// Frontend (Svelte) service

local compose = import '../compose/index.libsonnet';
local projDir = std.extVar('projDir');

// Add the service to a docker compose
function(file)
  file.addService(
    'front',
    // Frontend service configuration using compose builder
    compose.service()
    .withImage('zannabianca1997/${COMPOSE_PROJECT_NAME}-front')
    .withContainerName('${COMPOSE_PROJECT_NAME}-front')
    .withRestart('always')
    .withBuild(
      compose.build()
      .withContext(projDir)
      .withDockerfile('./docker/Dockerfile.sv')
      .withTarget('dev')
    )
    .addPort('3000:3000')
    .addVolume(projDir + '/web-ui/src:/app/src:ro')
    .addVolume(projDir + '/web-ui/static:/app/static:ro')
    + {
      specializeFor:: function(enviroment)
        if enviroment == 'dev' then self
        // service front exists only in dev
        else null,
    }
  )
