# `apelle` 2.0.0-alpha

> A communist music queue

`apelle` is a service for handling a shared music queue.
Users can insert songs in the queues, and upvote them to push them upward. `apelle`
will track the position of each song in the queue, and the position of the currently
playing song.

It also fetch the song data from the sources (for now, only Youtube is supported).
Users provides only the minimal necessary to identify the song (e.g. the youtube video ID).

## Local ports

| Service  | Port |
|----------|------|
| postgres | 5432 |
| users    | 8081 |

## Services

### `users`
