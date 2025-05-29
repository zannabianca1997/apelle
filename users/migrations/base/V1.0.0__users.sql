-- A user of the service
CREATE TABLE apelle_user (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);

-- Global role of the user
CREATE TABLE apelle_global_role (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL
);

-- Many-to-many relation between user and global role
CREATE TABLE apelle_user_global_role (
    user_id UUID NOT NULL REFERENCES apelle_user
        ON DELETE CASCADE,
    global_role_id UUID NOT NULL REFERENCES apelle_global_role
        ON DELETE CASCADE,
    PRIMARY KEY (user_id, global_role_id)
);

-- Which global roles can grant other global roles
CREATE TABLE apelle_global_role_can_grant (
    global_role_id UUID NOT NULL REFERENCES apelle_global_role
        ON DELETE CASCADE,
    can_grant_global_role_id UUID NOT NULL REFERENCES apelle_global_role
        ON DELETE CASCADE,
    PRIMARY KEY (global_role_id, can_grant_global_role_id)
);

-- Which global roles can remove other global roles
CREATE TABLE apelle_global_role_can_remove (
    global_role_id UUID NOT NULL REFERENCES apelle_global_role
        ON DELETE CASCADE,
    can_remove_global_role_id UUID NOT NULL REFERENCES apelle_global_role
        ON DELETE CASCADE,
    PRIMARY KEY (global_role_id, can_remove_global_role_id)
);