# `apelle-common`

This is a Rust library to provide a common base for all the service. It wraps
the main of the services, that must only provide the `axum`
[`Router`](https://docs.rs/axum/latest/axum/struct.Router.html) that serves the
exposed API.

It handles mainly configuration and logging, ensuring uniformity in both. It
also provide some helper methods, like date parsing, common error types and
auth header definition.

Generally if something is shared between most services, this is the place to put
it.

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

## Logging

The services use the [`tracing`](https://docs.rs/tracing/latest/tracing/) crate
for logging. Two tracing subscribers are created, following the configuration:
one to console, the other to rotating log files.