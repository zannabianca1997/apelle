# Docker related content

This directory contains the files needed to dockerize the rust services.
`Dockerfile.rust` is a three-stages dockerfile that compile any of the `apelle`
services. Just set the `SERVICE` arg to the name of the service, and build using
the repository root at the build context.

At the end of the build the `Apelle.docker.<profile>.toml` is copied into the
image and renamed to `Apelle-<service name>.toml`. It configures the service to
listen to port 8080, and for local developement to log to the rotating logs. The
rest of the configuration must be provided via other methods (see the
configuration options provided by the `apelle-common` crate).

By setting the `PROFILE` arg to `release` the services will be compiled to
release mode, getting them ready for prod.