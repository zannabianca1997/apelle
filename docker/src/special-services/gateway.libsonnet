// Nginx gateway service

local compose = import '../compose/index.libsonnet';
local projDir = std.extVar('projDir');
local deploy = import 'deploy.json';

local image = deploy.registry.user + '/${COMPOSE_PROJECT_NAME}-gateway';

local service =
  compose.service()
  .withContainerName('${COMPOSE_PROJECT_NAME}-gateway')
  .withRestart('always')
  .addDependsOn('users')
  .addDependsOn('songs')
  .addDependsOn('queues')
  .addDependsOn('configs')
  +
  {
    // Gateway is very different between prod and dev
    specializeFor:: function(environment)
      local actualService = if environment == 'dev' then
        self.withImage('nginx:1.27-alpine')
        .addVolume(projDir + '/gateway/inbound:/etc/nginx/inbound:ro')
        .addVolume(projDir + '/gateway/outbound:/etc/nginx/outbound:ro')
        .addVolume(projDir + '/gateway/nginx-dev.conf:/etc/nginx/nginx.conf:ro')
        .addDependsOn('front')
        .addPort('8080:8080')
      else if environment == 'build' then
        self.withImage(image)
        .withBuild(
          compose.build()
          .withContext(projDir)
          .withDockerfile('./docker/Dockerfile.sv')
          .withTarget('prod')
        )
      else
        self.withImage(image)
        .addPort('80:8080');


      // Remove anything not related to the build if the enviroment is a build one
      local withoutRunningArgs = if environment == 'build' then
        {
          build: actualService.build,
          image: actualService.image,
        }
      else
        actualService;

      withoutRunningArgs,
  };

function(file) file.addService('gateway', service)
