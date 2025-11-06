// Redis/Valkey cache and pubsub service

local compose = import '../compose/index.libsonnet';

// Add the service to a docker compose
function(file)
  file.addService(
    'cache-pubsub',
    // Cache/PubSub service configuration using compose builder
    compose.service()
    .withImage('valkey/valkey:8.1-alpine')
    .withContainerName('${COMPOSE_PROJECT_NAME}-cache-pubsub')
    .withRestart('always')
    .withCommand(['valkey-server', '--save', '', '--appendonly', 'no', '--maxmemory', '256mb', '--maxmemory-policy', 'allkeys-lru'])
    .addVolume('cache-data:/data')
    + {
      // delete the service if in a build enviroment
      specializeFor:: function(enviroment) if enviroment != 'build' then self else null,
    }
  )
  .addVolume(
    'cache-data',
    compose.volume().withName('${COMPOSE_PROJECT_NAME}-cache-data')
  )
