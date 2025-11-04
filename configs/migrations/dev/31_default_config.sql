SET CONSTRAINTS ALL DEFERRED;

DO $$
DECLARE
   admin_role_id UUID;
   voter_role_id UUID;
   banned_role_id UUID;
   observer_role_id UUID;
   -- Default config ID
   default_config_id UUID := '00000000-0000-0000-0000-000000000000';
BEGIN

   -- ADMIN role: full control
   INSERT INTO queue_user_role (config_id, name, max_likes)
   VALUES (default_config_id, 'ADMIN', 12997)
   RETURNING id INTO admin_role_id;

   -- VOTER role: enqueue, like, auto-next
   INSERT INTO queue_user_role (config_id, name, max_likes)
   VALUES (default_config_id, 'VOTER', 4)
   RETURNING id INTO voter_role_id;

   -- BANNED role: no action
   INSERT INTO queue_user_role (config_id, name, max_likes)
   VALUES (default_config_id, 'BANNED', 0)
   RETURNING id INTO banned_role_id;

   -- OBSERVER role: only auto-next
   INSERT INTO queue_user_role (config_id, name, max_likes)
   VALUES (default_config_id, 'OBSERVER', 0)
   RETURNING id INTO observer_role_id;


   INSERT INTO queue_user_role_permission (role_id, permission)
   VALUES
      -- Admin ( full control )
      (admin_role_id, 'GET_QUEUE'),
      (admin_role_id, 'DELETE_QUEUE'),
      (admin_role_id, 'CONFIGURE_QUEUE'),
      (admin_role_id, 'LIKE_SONG'),
      (admin_role_id, 'REMOVE_SONG'),
      (admin_role_id, 'BAN_SONG'),
      (admin_role_id, 'UNBAN_SONG'),
      (admin_role_id, 'ENQUEUE_SONG'),
      (admin_role_id, 'PLAY_SONG'),
      (admin_role_id, 'PAUSE_SONG'),
      (admin_role_id, 'NEXT_SONG'),
      (admin_role_id, 'AUTO_NEXT_SONG'),
      (admin_role_id, 'BAN_USER'),
      (admin_role_id, 'UNBAN_USER'),
      (admin_role_id, 'REMOVE_USER'),
      -- Voter (can like and enqueue songs)
      (voter_role_id, 'GET_QUEUE'),
      (voter_role_id, 'LIKE_SONG'),
      (voter_role_id, 'ENQUEUE_SONG'),
      (voter_role_id, 'AUTO_NEXT_SONG'),
      -- Banned (no action available)
      -- Observer (can only keep the queue running)
      (observer_role_id, 'GET_QUEUE'),
      (observer_role_id, 'AUTO_NEXT_SONG');


   -- Adnins can grant all roles
   INSERT INTO queue_user_grant_roles (role_id, granted_role_id)
   VALUES
      (admin_role_id, admin_role_id),
      (admin_role_id, voter_role_id),
      (admin_role_id, banned_role_id),
      (admin_role_id, observer_role_id);

   -- Adnins can remove all roles, except admins
   INSERT INTO queue_user_remove_roles (role_id, removed_role_id)
   VALUES
      (admin_role_id, voter_role_id),
      (admin_role_id, banned_role_id),
      (admin_role_id, observer_role_id);


   -- Default config
   INSERT INTO queue_config (id, creator_role_id, default_role_id, banned_role_id, autolike)
   VALUES
      (default_config_id, admin_role_id, voter_role_id, banned_role_id, true);

END $$;