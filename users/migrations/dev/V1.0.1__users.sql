-- Add admin role
INSERT INTO apelle_global_role (name) 
VALUES 
    ('admin');


INSERT INTO apelle_global_role_can_grant (global_role_id, can_grant_global_role_id) 
VALUES ( 
    (SELECT id FROM apelle_global_role WHERE name = 'admin'), 
    (SELECT id FROM apelle_global_role WHERE name = 'admin')
);