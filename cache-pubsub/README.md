# `cache-pubsub`

A valkey instance, used as a cache and a pub-sub

## Cache
While the data stored in the cache are all temporary, they should never be data
that cannot be lost at a moment notice. Still, services depending on this should
be restarted if this service even fail.

Cache keys are prefixed with the `apelle:<service-name>:` namespace to ensure
uniquedness. No cache key is shared, as inter-service communication is made via
http or pub-sub.