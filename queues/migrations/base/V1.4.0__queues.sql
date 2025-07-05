-- Main queue entity
CREATE TABLE queue (
    -- Internal id of the queue
    id UUID PRIMARY KEY
        DEFAULT gen_random_uuid(),
    
    -- The code of the queue
    code TEXT NOT NULL UNIQUE,
    
    -- The id of the song that is currently playing
    current_song UUID
        REFERENCES song
        ON UPDATE CASCADE
        ON DELETE RESTRICT
        DEFAULT NULL,

    -- Current song position in seconds
    current_song_position INTEGER 
        DEFAULT NULL,
    
    -- Time in the past the song should have started to reach the current position
    current_song_start_at 
        TIMESTAMP WITH TIME ZONE 
        DEFAULT NULL,
    
    -- Either no current song, or it is either started or stopped
    CONSTRAINT song_is_either_started_or_stopped CHECK ( 
        (
            -- The current song is null
            (current_song IS NULL)
            AND (current_song_start_at IS NULL)
            AND (current_song_position IS NULL)
        )
        OR (
            -- Only one of the time reference is filled in
            (current_song IS NOT NULL)
            AND (
                (current_song_start_at IS NULL) <> (current_song_position IS NULL)
            )
        )
    ),

    -- Random id changing each time the three columns are acted upon
    player_state_id UUID NOT NULL
        DEFAULT gen_random_uuid(),

    -- Configuration
    config_id UUID NOT NULL
        REFERENCES queue_config
        ON DELETE RESTRICT
        ON UPDATE CASCADE
        DEFAULT '00000000-0000-0000-0000-000000000000',

    created TIMESTAMP WITH TIME ZONE NOT NULL
        DEFAULT NOW(),
    updated TIMESTAMP WITH TIME ZONE NOT NULL
        DEFAULT NOW()
);

CREATE TABLE queue_user (
    queue_id UUID NOT NULL
        REFERENCES queue
        ON UPDATE CASCADE
        ON DELETE CASCADE,

    user_id UUID NOT NULL
        REFERENCES apelle_user
        ON UPDATE CASCADE
        ON DELETE CASCADE,

    -- Whether the user has autolike turned on, regardless of the config
    autolike BOOLEAN
        DEFAULT NULL,

    -- Role of the user
    role_id UUID NOT NULL
        REFERENCES queue_user_role
        ON UPDATE CASCADE
        ON DELETE RESTRICT,

    -- First time the user connected
    created TIMESTAMP WITH TIME ZONE
        NOT NULL
        DEFAULT NOW(),

    -- Last time the user was seen
    last_seen TIMESTAMP WITH TIME ZONE
        NOT NULL
        DEFAULT NOW(),

    PRIMARY KEY (queue_id, user_id)
);

CREATE TABLE queued_song (
    queue_id UUID NOT NULL
        REFERENCES queue
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    song_id UUID NOT NULL
        REFERENCES song
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    CONSTRAINT pk_queued_song 
    PRIMARY KEY (queue_id, song_id),

    queued_at 
        TIMESTAMP WITH TIME ZONE 
        NOT NULL 
        DEFAULT NOW(),

    queued_by UUID NOT NULL,
    
    FOREIGN KEY (queue_id, queued_by)
        REFERENCES queue_user
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE TABLE likes (

    queue_id UUID NOT NULL,

    song_id UUID NOT NULL,

    FOREIGN KEY (queue_id, song_id)
        REFERENCES queued_song
        ON UPDATE CASCADE
        ON DELETE CASCADE,

    user_id UUID NOT NULL,

    FOREIGN KEY (queue_id, user_id)
        REFERENCES queue_user
        ON UPDATE CASCADE
        ON DELETE CASCADE,

    -- Time the like was given
    given_at TIMESTAMP WITH TIME 
        ZONE NOT NULL
        DEFAULT NOW(),

    PRIMARY KEY (queue_id, song_id, user_id, given_at),

    -- Number of likes this row represents
    count SMALLINT 
        NOT NULL 
        DEFAULT 1,
    CHECK (count > 0)
);

-- Functions

-- Change the player state id and last modified date of a queue

CREATE TYPE player_state_id_and_updated AS (
    player_state_id UUID,
    updated TIMESTAMP WITH TIME ZONE
);

CREATE FUNCTION update_queue(
    p_queue_id UUID
)
RETURNS player_state_id_and_updated
LANGUAGE plpgsql
AS $$
DECLARE
    _result player_state_id_and_updated;
BEGIN
    UPDATE queue
    SET 
        updated = NOW(),
        player_state_id = gen_random_uuid()
    WHERE id = p_queue_id
    RETURNING player_state_id, updated 
    INTO _result;

    RETURN _result;
END;
$$;

-- Find the oldest like given by a user in a queue,
-- except on a specific song, and remove it.

CREATE FUNCTION remove_oldest_like(
    p_queue_id UUID, p_user_id UUID, p_excluded_song_id UUID
)
RETURNS UUID
LANGUAGE plpgsql
AS $$
DECLARE
    _target_queue_id UUID;
    _target_song_id UUID;
    _target_user_id UUID;
    _target_given_at TIMESTAMP WITH TIME ZONE;
    _current_count SMALLINT;
BEGIN
    -- Find the oldest like not on the excluded song
    SELECT
        queue_id,
        song_id,
        user_id,
        given_at,
        count
    INTO
        _target_queue_id,
        _target_song_id,
        _target_user_id,
        _target_given_at,
        _current_count
    FROM
        likes
    WHERE
        queue_id = p_queue_id AND
        user_id = p_user_id AND
        song_id != p_excluded_song_id
    ORDER BY
        given_at ASC
    LIMIT 1;

    -- Check if a row was actually found by the SELECT statement
    IF FOUND THEN
        -- Remove the row if it was a single like, otherwise decrement the count
        IF _current_count = 1 THEN
            DELETE FROM likes
            WHERE
                queue_id = _target_queue_id AND
                song_id = _target_song_id AND
                user_id = _target_user_id AND
                given_at = _target_given_at;
        ELSE
            UPDATE likes
            SET count = count - 1
            WHERE
                queue_id = _target_queue_id AND
                song_id = _target_song_id AND
                user_id = _target_user_id AND
                given_at = _target_given_at;
        END IF;

        -- If a like was found and processed (either deleted or count
        -- decremented), return the song_id.
        RETURN _target_song_id;
    ELSE
        -- If no like was found, return NULL.
        RETURN NULL;
    END IF;
END;
$$;