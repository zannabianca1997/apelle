# `db`

Here you'll find an instance of `PostgreSQL` configured to work as a common
database for all services.

A `.env` must be provided with the value for `DB_USER` and `DB_PASSWORD`.

There is also another service, `apelle-migrator`, that collects the migration of all
services and applies them to the databse before the other services can start up.