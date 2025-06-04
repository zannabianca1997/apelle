# `apelle` 2.0.0-alpha

> A communist music queue

`apelle` is a service for handling a shared music queue. Users can insert songs
in the queues, and upvote them to push them upward. `apelle` will track the
position of each song in the queue, and the position of the currently playing
song.

It also fetch the song data from the sources (for now, only Youtube is
supported). Users provides only the minimal necessary to identify the song (e.g.
the youtube video ID).

## Ports for local developement

| Service       | Port |
|---------------|------|
| db            | 5432 |
| cache-pubsub  | 6379 |
| gateway       | 8080 |
| users         | 8081 |
| songs         | 8082 |
| songs-youtube | 8091 |

## Configuration
Every service can take configuration from multiple sources. In order,
configuration keys are searched:
- Cli arguments: `-C serve.port`
- Enviroment variables: `APELLE__SERVE__PORT`
- A TOML file provided with `-c`
- The `global` table of a TOML file called `Apelle.toml` in the current
  directory or in an ancestor
- A TOML file called `Apelle-<service name>.toml` in the current directory or in
  an ancestor
- The `<service name>` table of `Apelle.toml`
- The `default` table of `Apelle.toml`
- The default values


## Services

## `db`
A simple postgres instance. It is used to store all permanent data

## `migrator`
A container used to handle migrations. It is configured to migrate `db` at
startup, with the migration collected from all the others services

## `gateway`
An nginx instance, working as the entry point to the costellation. It has two
main functions: handling authentication, and routing to the various services. On
an incoming request it forwards the request headers with a `GET` on the `/auth`
endpoint of the `users` services. If the request authenticate with success, the
headers returned by the auth services are added to the request and the final
result is forwarded to the `/public` endpoint of the service.

## `cache-pubsub`
A valkey instance working a double function. First, it provide a common pub-sub
to communicate real-time events (likes and song additions). Second, it serves as
a cache for volatile data like youtube searches and registered providers.

### `users`
User service. Handles authentication, and user management.

### `songs`
Song service. Keep track of songs, sources, and the providers for each source.
Route every request of a song to the registered providers.

### `songs-<source>`
Song providers. Handle communication with the given source. Register themselves
at runtime on the `songs` service.
