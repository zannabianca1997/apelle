-- Youtube specific data on a song
CREATE TABLE youtube_song (
    -- Id of the main entity
    id UUID PRIMARY KEY
        REFERENCES song
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    -- Id of the video
    video_id TEXT NOT NULL UNIQUE,
    -- Etag at the time of the fetching
    etag TEXT,
    -- Date the video data were fetched
    fetched TIMESTAMP WITH TIME ZONE 
        NOT NULL
        DEFAULT NOW()
);

-- Thumbnails
CREATE TABLE youtube_thumbnail (
    -- Song this thumbnail belongs to
    song_id UUID NOT NULL
        REFERENCES youtube_song
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    -- Size of the thumbnail
    height INTEGER NOT NULL,
    width INTEGER NOT NULL,
    -- URL of the thumbnail
    url TEXT NOT NULL,

    -- Name of the thumbnail size
    -- (e.g. "default", "mqdefault", "hqdefault", "maxresdefault")
    size TEXT NOT NULL,
    PRIMARY KEY (song_id, size)
);