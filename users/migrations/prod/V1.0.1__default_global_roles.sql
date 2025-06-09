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