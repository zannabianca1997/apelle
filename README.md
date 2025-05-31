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
| songs    | 8082 |

## Configuration
Every service can take configuration from multiple sources.
In order, configuration keys are searched:
- Cli arguments: `-C serve.port`
- Enviroment variables: `APELLE__SERVE__PORT`
- A TOML file provided with `-c`
- The `global` table of a TOML file called `Apelle.toml` in the current directory or in an ancestor
- A TOML file called `Apelle-<service name>.toml` in the current directory or in an ancestor
- The `<service name>` table of `Apelle.toml`
- The `default` table of `Apelle.toml`
- The default values


## Services

## `db`
A simple postgres instance. It is used to store all permanent data

## `flyway`
A container used to handle migrations. It is configured to migrate `db` at 
startup, with the migration collected from all the others services

## `gateway`
The entry point to the costellation. It has two main functions: handling
authentication, and routing to the various services. On an incoming request
it forwards the request headers with a `GET` on the `/auth` endpoint of the 
`users` services. If the request authenticate with success, the headers
returned by the auth services are added to the request and the final result
is forwarded to the `/public` endpoint of the service.

### `users`
User service. Handles authentication, and user management.