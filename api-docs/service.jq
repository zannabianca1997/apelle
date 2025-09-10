.extern as $extern |
.service as $service |
.spec |
# Remove general keys that will be provided aside
del(.openapi, .info) |

# Editing the paths
.paths |= with_entries(
    # Select all the public paths
    select(.key | startswith("/public")) |
    # Remove the /public prefix and add the specified extern
    .key |= sub("^/public"; $extern)
) |

# Editing the paths
.paths |= map_values(
    map_values(
        # Remove the public tag and prepend the service to the operationId
        (
            .tags |= map(select(. != "public")) |
            .operationId |= $service + "_" + .
        )
    )
) |

# Get all unique tags from the paths
([.paths | values[] | values[] | .tags] | flatten | unique) as $used_tags |
# Keep only the tags that are in the paths
.tags |= map(select(.name | IN($used_tags[])))