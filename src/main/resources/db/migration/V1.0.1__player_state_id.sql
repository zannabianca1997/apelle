-- Create the state_id column
ALTER TABLE IF EXISTS queue 
ADD IF NOT EXISTS player_state_id UUID NOT NULL
DEFAULT (gen_random_uuid());