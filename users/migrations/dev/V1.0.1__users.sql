-- Add admin role
WITH admin_role AS (
    -- Add the role, returning the id
    INSERT INTO apelle_global_role (name) 
    VALUES ('admin')
    RETURNING id
)
-- Add as a rule: admins can create other admins
INSERT INTO apelle_global_role_can_grant (global_role_id, can_grant_global_role_id) 
SELECT id, id FROM admin_role;

-- Add test users
INSERT INTO apelle_user (name, password) 
VALUES
    ('admin', '$argon2i$v=19$m=16,t=2,p=1$cHlYZFNPWTYycUpRdTlTcg$x73G6++j//8exYfkR1nbug'),
    ('user',  '$argon2i$v=19$m=16,t=2,p=1$cHlYZFNPWTYycUpRdTlTcg$x73G6++j//8exYfkR1nbug');

INSERT INTO apelle_user_global_role (user_id, global_role_id) 
VALUES
    (
        (SELECT id FROM apelle_user WHERE name = 'admin'), 
        (SELECT id FROM apelle_global_role WHERE name = 'admin')
);