-- Create the config tables

CREATE TABLE queue_user_role (
    id UUID PRIMARY KEY, 
    name VARCHAR(255) NOT NULL, 
    max_likes SMALLINT NOT NULL, 
    permissions_delete BOOLEAN NOT NULL, 
    permissions_queue_ban BOOLEAN NOT NULL, 
    permissions_queue_enqueue BOOLEAN NOT NULL, 
    permissions_queue_like BOOLEAN NOT NULL, 
    permissions_queue_next BOOLEAN NOT NULL, 
    permissions_queue_remove BOOLEAN NOT NULL, 
    permissions_queue_start BOOLEAN NOT NULL, 
    permissions_queue_stop BOOLEAN NOT NULL, 
    permissions_queue_users_ban BOOLEAN NOT NULL, 
    permissions_queue_users_remove BOOLEAN NOT NULL
);

CREATE TABLE permissions_queue_users_grant_roles (
    role_id UUID NOT NULL REFERENCES queue_user_role ON DELETE CASCADE,
    granted_role_id UUID NOT NULL REFERENCES queue_user_role ON DELETE CASCADE,
    primary key (role_id, granted_role_id)
);

CREATE TABLE permissions_queue_users_remove_roles (
    role_id UUID NOT NULL REFERENCES queue_user_role ON DELETE CASCADE,
    removed_role_id UUID NOT NULL REFERENCES queue_user_role ON DELETE CASCADE,
    primary key (role_id, removed_role_id)
);

CREATE TABLE queue_config (
    id UUID PRIMARY KEY, 
    autolike BOOLEAN NOT NULL,
    banned_role_id UUID NOT NULL REFERENCES queue_user_role,
    creator_role_id UUID NOT NULL REFERENCES queue_user_role, 
    default_role_id UUID NOT NULL REFERENCES queue_user_role
);

-- Add the current configs

INSERT INTO queue_user_role
VALUES
    (gen_random_uuid(), 'CREATOR', 32000,  true,  true,  true,  true,  true,  true,  true,  true,  true,  true),
    (gen_random_uuid(), 'BANNED',      0, false, false, false, false, false, false, false, false, false, false),
    (gen_random_uuid(), 'OBSERVER',    0, false, false, false, false, false, false, false, false, false, false),
    (gen_random_uuid(), 'PLAYER',      0, false, false, false, false,  true, false,  true,  true, false, false),
    (gen_random_uuid(), 'VOTER',       4, false, false,  true,  true, false, false, false, false, false, false);

-- Creators can grant all roles, and remove then except other creators
INSERT INTO permissions_queue_users_grant_roles
SELECT 
    (SELECT id FROM queue_user_role WHERE name = 'CREATOR') AS role_id,
    id AS granted_role_id
FROM queue_user_role
ON CONFLICT DO NOTHING;

INSERT INTO permissions_queue_users_remove_roles
SELECT 
    (SELECT id FROM queue_user_role WHERE name = 'CREATOR') AS role_id,
    id AS removed_role_id
FROM queue_user_role
WHERE name != 'CREATOR'
ON CONFLICT DO NOTHING;

-- Everyone that can ban, can grant the banned role
INSERT INTO permissions_queue_users_grant_roles
SELECT 
    id AS role_id,
    (SELECT id FROM queue_user_role WHERE name = 'BANNED') AS granted_role_id
FROM queue_user_role
WHERE permissions_queue_users_ban = true
ON CONFLICT DO NOTHING;

INSERT INTO queue_config
VALUES
    (
        '00000000-0000-0000-0000-000000000000', 
        true, 
        (SELECT id FROM queue_user_role WHERE name = 'BANNED'), 
        (SELECT id FROM queue_user_role WHERE name = 'CREATOR'), 
        (SELECT id FROM queue_user_role WHERE name = 'VOTER')
    );

-- Adding config column to queue table
ALTER TABLE queue 
ADD COLUMN config_id UUID NOT NULL 
DEFAULT '00000000-0000-0000-0000-000000000000' 
REFERENCES queue_config;

-- Change the role column to a new one referencing the newly created table
ALTER TABLE queue_user 
ADD COLUMN role_id UUID
REFERENCES queue_user_role;

UPDATE queue_user
SET role_id = (SELECT id FROM queue_user_role WHERE name = 'CREATOR')
WHERE role = 'PLAYER';

UPDATE queue_user
SET role_id = (SELECT id FROM queue_user_role WHERE name = 'VOTER')
WHERE role = 'VOTER';

UPDATE queue_user
SET role_id = (SELECT id FROM queue_user_role WHERE name = 'BANNED')
WHERE role = 'OBSERVER';

ALTER TABLE queue_user
ALTER COLUMN role_id 
SET NOT NULL;

ALTER TABLE queue_user
DROP COLUMN role;