. as $input
# Merge all fields from the global object with the merged paths and tags
| ($input.global // {}) + {
    # Combine paths from global and all apis
    paths: (($input.global.paths // {}) + ($input.apis | reduce .[] as $item ({}; .paths += ($item.paths // {})))).paths,
    # Combine components from global and all apis
    components: {
        schemas: (($input.global.components.schemas // {}) + ($input.apis | reduce .[] as $item ({}; .components.schemas += ($item.components.schemas // {})))).components.schemas
    },
    # Combine tags from global and all apis, then make them unique
    tags: (($input.global.tags // []) + ($input.apis | reduce .[] as $item ([]; . + ($item.tags // [])))) | unique_by(.name)
}