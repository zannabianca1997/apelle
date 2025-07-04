CREATE TYPE YoutubeThumbnailSize AS ENUM('DEFAULT', 'HIGH', 'MAXRES', 'MEDIUM', 'STANDARD');

CREATE CAST (VARCHAR AS YoutubeThumbnailSize)
WITH INOUT AS IMPLICIT;

CREATE CAST (YoutubeThumbnailSize AS VARCHAR)
WITH INOUT AS IMPLICIT;

CREATE TABLE apelle_user (
    id UUID NOT NULL,
    name VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    roles VARCHAR(255) ARRAY NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE likes (
    count SMALLINT NOT NULL,
    given_at TIMESTAMP(6) WITH TIME ZONE NOT NULL,
    queue_id UUID NOT NULL,
    song_id UUID NOT NULL,
    user_id UUID NOT NULL,
    PRIMARY KEY (given_at, queue_id, song_id, user_id)
);

CREATE TABLE queue (
    current_song_position NUMERIC(21, 0),
    current_song_starts_at TIMESTAMP(6) WITH TIME ZONE,
    current_song UUID,
    id UUID NOT NULL,
    code VARCHAR(255) NOT NULL UNIQUE,
    PRIMARY KEY (id),
    CONSTRAINT song_is_either_started_or_stopped CHECK ( -- Either the current song is started or it's stopped
        (
            -- The current song is null
            (current_song IS NULL)
            AND (current_song_starts_at IS NULL)
            AND (current_song_position IS NULL)
        )
        OR (
            -- Only one of the time reference is filled in
            (current_song IS NOT NULL)
            AND (
                (current_song_starts_at IS NULL) <> (current_song_position IS NULL)
            )
        )
    )
);

CREATE TABLE queue_user (
    queue_id UUID NOT NULL,
    user_id UUID NOT NULL,
    role VARCHAR(255) NOT NULL,
    PRIMARY KEY (queue_id, user_id)
);

CREATE TABLE queued_song (
    queued_at TIMESTAMP(6) WITH TIME ZONE NOT NULL,
    queue_id UUID NOT NULL,
    song_id UUID NOT NULL,
    PRIMARY KEY (queue_id, song_id)
);

CREATE TABLE song (
    duration NUMERIC(21, 0) NOT NULL,
    id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE youtube_song (
    id UUID NOT NULL,
    video_id VARCHAR(255) NOT NULL UNIQUE,
    PRIMARY KEY (id)
);

CREATE TABLE youtube_thumbnail (
    height INTEGER NOT NULL,
    width INTEGER NOT NULL,
    song_id UUID NOT NULL,
    url VARCHAR(255) NOT NULL,
    size YoutubeThumbnailSize NOT NULL,
    PRIMARY KEY (song_id, size)
);

ALTER TABLE IF EXISTS likes
ADD CONSTRAINT FKesxvn09aod5a9513l67fflvbx FOREIGN KEY (queue_id) REFERENCES queue ON DELETE CASCADE;

ALTER TABLE IF EXISTS likes
ADD CONSTRAINT FK3ducolygbx3s2eejaic8uw2cc FOREIGN KEY (song_id) REFERENCES song ON DELETE CASCADE;

ALTER TABLE IF EXISTS likes
ADD CONSTRAINT FK7gs14x7pndcr80d4hwtoy6010 FOREIGN KEY (user_id) REFERENCES apelle_user ON DELETE CASCADE;

ALTER TABLE IF EXISTS queue
ADD CONSTRAINT FKoc9ha8v8lr2p52af0a9swiglj FOREIGN KEY (current_song) REFERENCES song;

ALTER TABLE IF EXISTS queue_user
ADD CONSTRAINT FKsew5fdyfmf7c2ukvtf2opl17s FOREIGN KEY (queue_id) REFERENCES queue ON DELETE CASCADE;

ALTER TABLE IF EXISTS queue_user
ADD CONSTRAINT FKfmhqiyrplccxxoqipl31vnu8i FOREIGN KEY (user_id) REFERENCES apelle_user ON DELETE CASCADE;

ALTER TABLE IF EXISTS queued_song
ADD CONSTRAINT FKl7muexxomnqrv6ygoo66vqtjd FOREIGN KEY (queue_id) REFERENCES queue ON DELETE CASCADE;

ALTER TABLE IF EXISTS queued_song
ADD CONSTRAINT FKs4rh77j2p2wuya8h3kqxwqh0p FOREIGN KEY (song_id) REFERENCES song ON DELETE CASCADE;

ALTER TABLE IF EXISTS youtube_song
ADD CONSTRAINT FKsg3r111rp1qq6i5l0lt27sm6x FOREIGN KEY (id) REFERENCES song ON DELETE CASCADE;

ALTER TABLE IF EXISTS youtube_thumbnail
ADD CONSTRAINT FKmjlg5uf39kkfr55q1m0riwhub FOREIGN KEY (song_id) REFERENCES youtube_song ON DELETE CASCADE;