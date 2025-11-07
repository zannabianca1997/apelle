// Nginx gateway service

local compose = import '../compose/index.libsonnet';
local projDir = std.extVar('projDir');
local deploy = import 'deploy.json';

local image = deploy.registry.user + '/${COMPOSE_PROJECT_NAME}-gateway';

local gatewayService =
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
        .addPort('80:80')
        .addPort('443:443')
        .addVolume('certbot-www:/var/www/certbot/:ro')
        .addVolume('certbot-conf:/etc/letsencrypt/:ro');


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

local certbotService =
  compose.service()
  .withImage('certbot/certbot:latest')
  .withContainerName('${COMPOSE_PROJECT_NAME}-certbot')
  .addVolume('certbot-www:/var/www/certbot/:rw')
  .addVolume('certbot-conf:/etc/letsencrypt/:rw')
  + {
    specializeFor:: function(enviroment)
      if enviroment == 'prod' then self
      // service certbot exists only in prod
      else null,
  }
;

function(file) file.addService('gateway', gatewayService).addService('certbot', certbotService)
