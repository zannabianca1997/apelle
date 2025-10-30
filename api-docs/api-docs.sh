#!/bin/env bash

: ${PYTHON:=python3}
: ${JQ:=jq}


NAME=$(basename $0)
DEFAULT_SERVICES="$(dirname $0)/services.csv"
DEFAULT_GLOBAL="$(dirname $0)/global.yml"
DEFAULT_OUTPUT="$(dirname $0)/../openapi.yml"

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
JQ_SERVICE_SCRIPT="$SCRIPT_DIR/service.jq"
JQ_MERGE_SCRIPT="$SCRIPT_DIR/merge.jq"

help() {
cat <<END
Build the global api docs from the ones served by the single services

Usage: $NAME [-h|--help] [-s|--services <SERVICES.CSV>] [-g|--global <GLOBAL.YML>] [--] [<OUTPUT>]

Arguments:
    OUTPUT  The output file (default: $DEFAULT_OUTPUT)

Options:
    -h,--help       Print this help
    -s,--services   The services csv file   (default: $DEFAULT_SERVICES)
    -g,--global     The global openapi file (default: $DEFAULT_GLOBAL)
END
}

parse_opts() {
    SERVICES="$DEFAULT_SERVICES"
    OUTPUT="$DEFAULT_OUTPUT"
    GLOBAL="$DEFAULT_GLOBAL"
    
    # Define the short and long options
    SHORT_OPTS="hsg:"
    LONG_OPTS="help,services,global:"

    # Parse the options
    PARSED_OPTS=$(getopt -o "$SHORT_OPTS" -l "$LONG_OPTS" --name "$NAME" -- "$@")

    # Check for parsing errors
    if [ $? -ne 0 ]; then
        help
        exit 1
    fi

    # Set the parsed options
    eval set -- "$PARSED_OPTS"

    # Process the options
    while true; do
        case "$1" in
            -h|--help)
                help
                exit 0
                ;;
            -s|--services)
                SERVICES="$2"
                shift 2
                ;;
            -g|--global)
                GLOBAL="$2"
                shift 2
                ;;
            --)
                shift
                break
                ;;
            *)
                echo "Internal error"
                exit 1
                ;;
        esac
    done 

    if [ $# -ne 0 ]
    then
        OUTPUT=$1
        shift
    fi


    if [ $# -ne 0 ]
    then
        echo "Unrecognized arguments: $@"
        help
        exit 1
    fi
}

global() {
    yaml2json < "$GLOBAL"
}

yaml2json() {
    "$PYTHON" -c 'import json, sys, yaml ; y=yaml.safe_load(sys.stdin.read()); print(json.dumps(y))'
}

json2yaml() {
    "$PYTHON" -c 'import sys, yaml, json; j=json.loads(sys.stdin.read()); print(yaml.safe_dump(j))'
}

service() {
    local service="$1"
    local url="$2"
    local extern="$3"

    printf "Processing ${service} at ${url}, mounted at ${extern}\n"

    # Check if the service is present
    local fetch_name=$(curl -s "$url/service/name")
    if [ "$service" != "$fetch_name" ]
    then
        printf "Service name mismatch: ${url} should host ${service}, but ${fetch_name} was found\n"
        return
    fi

    local api="{\"extern\": \"$extern\", \"service\": \"$service\", \"spec\": $(curl -s "$url/service/openapi.json")}"

    api="$("$JQ" -c -f "$JQ_SERVICE_SCRIPT" <<< "$api")"

    APIS="${APIS}${api},"
}

main() {
    parse_opts $@

    printf "Building the api docs from ${SERVICES} to ${OUTPUT}\n"

    APIS="{\"global\": $(global), \"apis\": ["

    while IFS=, read -r service url extern; do
        service "$service" "$url" "$extern"
    done < "$SERVICES"

    APIS="${APIS::-1}]}"

    local merged=$("$JQ" -c -f "$JQ_MERGE_SCRIPT" <<< "${APIS}")

    json2yaml <<< "$merged" > "$OUTPUT"
}


main $@
