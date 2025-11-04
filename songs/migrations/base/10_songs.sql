-- Known sources
CREATE TABLE source (
    id UUID PRIMARY KEY
        DEFAULT gen_random_uuid(),
    -- Identifier of the source (e.g. urn:apelle:sources:youtube)
    urn TEXT NOT NULL UNIQUE,
    -- Human readable name
    name TEXT NOT NULL,
    -- Date the source was added
    created TIMESTAMP WITH TIME ZONE 
        NOT NULL
        DEFAULT NOW(),
    -- Last time a provider for this source was heard
    last_heard TIMESTAMP WITH TIME ZONE
        DEFAULT NULL
);

-- Main table for the songs
CREATE TABLE song (
    -- Unique id of the song
    id UUID PRIMARY KEY
        DEFAULT gen_random_uuid(),
    -- Human readable name
    title TEXT NOT NULL,
    -- Duration in seconds
    duration INTEGER NOT NULL,
    -- Who added the song
    added_by UUID 
        REFERENCES apelle_user
        ON UPDATE CASCADE
        ON DELETE SET NULL
        DEFAULT NULL,
    -- Date the song was added
    created TIMESTAMP WITH TIME ZONE 
        NOT NULL
        DEFAULT NOW(),
    -- Last time the song was removed from a playlist
    last_removed_from_playlist 
        TIMESTAMP WITH TIME ZONE
        DEFAULT NULL,
    -- UUID of the source of the song
    source_id UUID NOT NULL
        REFERENCES source
        ON UPDATE CASCADE
        ON DELETE RESTRICT
);