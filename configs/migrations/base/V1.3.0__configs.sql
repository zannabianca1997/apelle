-- Create the config tables

CREATE TABLE queue_user_role (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    config_id UUID NOT NULL,
    name TEXT NOT NULL, 

    UNIQUE (config_id, name),

    max_likes SMALLINT NOT NULL,
    CHECK (max_likes >= 0)
);

-- Actions each user can perform
CREATE TYPE queue_user_action AS ENUM(
    'GET_QUEUE',
    'DELETE_QUEUE',
    'CONFIGURE_QUEUE',
    
    'REMOVE_SONG',
    'BAN_SONG',
    'UNBAN_SONG',
    'ENQUEUE_SONG',
    'PLAY_SONG',
    'PAUSE_SONG',
    'LIKE_SONG',
    'NEXT_SONG',
    'AUTO_NEXT_SONG',

    'BAN_USER',
    'UNBAN_USER',
    'REMOVE_USER'
);

CREATE CAST (VARCHAR AS queue_user_action)
WITH INOUT AS IMPLICIT;

CREATE CAST (queue_user_action AS VARCHAR)
WITH INOUT AS IMPLICIT;

CREATE TABLE queue_user_role_permission (
    role_id UUID NOT NULL
        REFERENCES queue_user_role
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    permission queue_user_action NOT NULL,

    PRIMARY KEY (role_id, permission)
);

CREATE TABLE queue_user_grant_roles (
    role_id UUID NOT NULL
        REFERENCES queue_user_role
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    granted_role_id UUID NOT NULL
        REFERENCES queue_user_role
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    PRIMARY KEY (role_id, granted_role_id)
);

CREATE TABLE queue_user_remove_roles (
    role_id UUID NOT NULL
        REFERENCES queue_user_role
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    removed_role_id UUID NOT NULL
        REFERENCES queue_user_role
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    PRIMARY KEY (role_id, removed_role_id)
);

CREATE TABLE queue_config (
    id UUID PRIMARY KEY 
        DEFAULT gen_random_uuid(),

    creator_role_id UUID NOT NULL
        REFERENCES queue_user_role
        ON DELETE RESTRICT
        ON UPDATE CASCADE,

    default_role_id UUID NOT NULL
        REFERENCES queue_user_role
        ON DELETE RESTRICT
        ON UPDATE CASCADE,

    banned_role_id UUID NOT NULL
        REFERENCES queue_user_role
        ON DELETE RESTRICT
        ON UPDATE CASCADE,

    autolike BOOLEAN NOT NULL 
        DEFAULT TRUE,

    created TIMESTAMP WITH TIME ZONE NOT NULL
        DEFAULT NOW(),

    updated TIMESTAMP WITH TIME ZONE NOT NULL
        DEFAULT NOW()
);

ALTER TABLE queue_user_role
    ADD CONSTRAINT queue_user_role_config_id_is_valid
        FOREIGN KEY (config_id)
        REFERENCES queue_config
        ON DELETE CASCADE
        ON UPDATE CASCADE
        DEFERRABLE;