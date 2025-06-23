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