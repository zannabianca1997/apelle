ALTER TABLE queued_song 
ADD COLUMN ref 
UUID NOT NULL UNIQUE
DEFAULT gen_random_uuid();

CREATE TABLE new_likes (
    given_at TIMESTAMP(6) WITH TIME ZONE NOT NULL, 
    count SMALLINT NOT NULL, CHECK (count > 0),
    user_id UUID NOT NULL REFERENCES apelle_user ON DELETE CASCADE, 
    queued_song_ref UUID NOT NULL REFERENCES queued_song(ref) ON DELETE CASCADE, 
    PRIMARY KEY (given_at, queued_song_ref, user_id)
);

-- Add the old likes to the new table, dropping the ones with an invalid count or reference

INSERT INTO new_likes
SELECT  
    likes.given_at, 
    likes.count, 
    likes.user_id, 
    queued_song.ref
FROM likes
INNER JOIN queued_song 
    ON likes.song_id = queued_song.song_id 
    AND likes.queue_id = queued_song.queue_id
WHERE likes.count > 0;

DROP TABLE likes;
ALTER TABLE new_likes RENAME TO likes;