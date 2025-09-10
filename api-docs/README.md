# OpenAPI helper tool

This is an helper tool to generate the OpenAPI specification for the project. It
is meant to run against a running copy of the project, and will update the yaml
file in the root of the project by merging all the APIs served by the various
services.

See `api-docs.sh --help` for more info.

## Adding informations to the specs

The file `global.yml` contains global info about the api. Edit that to add non
service-specific info.

## Adding a service

The file `services.csv` contains the list of services to merge, plus the url to
reach them and the public url they will be exposed at. Please notice that that
file will be read by a very simplistic bash script, so it is a bit strict on the
accepted syntax.