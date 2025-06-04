# `db`

Here you'll find an instance of `PostgreSQL` configured to work as a common
database for all services.

There is also another service, `migrator`, that collects the migration of all
services and applies them to the databse before the other services can start up.
It is currently an instance of [the official flyway
image](https://hub.docker.com/r/flyway/flyway).