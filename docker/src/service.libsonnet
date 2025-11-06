// Base class for an Apelle service

local compose = import 'compose/index.libsonnet';
local deploy = import 'deploy.json';

local projDir = std.extVar('projDir');

local composeServiceSpecializer = function(name, devPort) {
  name:: name,
  devPort:: devPort,

  specializeFor:: function(environment)
    // Add image info. Dev should never be pushed
    local withImage = if environment != 'dev' then
      self
      .withImage(deploy.registry.user + '/' + self.container_name)
    else
      self;

    // Add build info. Prod should never build
    local withBuild = if environment != 'prod' then
      withImage.withBuild(
        compose.build()
        .withContext(projDir)
        .withDockerfile('./docker/Dockerfile.rust')
        .addArg('SERVICE', self.name)
        .addArg('PROFILE', if environment == 'dev' then 'dev' else 'release')
      )
    else
      withImage;

    // Remove anything not related to the build if the enviroment is a build one
    local withoutRunningArgs = if environment == 'build' then
      {
        build: withBuild.build,
        image: withBuild.image,
      }
    else
      withBuild;

    // Add a dev port if the enviroment is a developer one. Connect the logs dir
    local withPort = if environment == 'dev' && devPort != null then
      withoutRunningArgs.addPort(devPort + ':8080')
    else withoutRunningArgs;

    local withLogs = if environment == 'dev' then
      withPort.addVolume(projDir + '/logs/:/app/logs')
    else withPort;

    withLogs,
};

{
  devPort: null,
  db: false,
  cache: false,
  pubsub: false,
  gateway: false,
  deps: [],
  secrets: [],

  // Create the service section for this service
  composeService:: function(name)
    local base = compose.service()
                 .withContainerName('${COMPOSE_PROJECT_NAME}-' + name)
                 .withRestart('always');

    // Adding dependencies

    // Global ones

    local withDb = if self.db then
      base.addDependsOn('db')
      .addDependsOn('migrator', 'service_completed_successfully')
    else
      base;
    local withCachePubSub = if self.cache || self.pubsub then
      withDb.addDependsOn('cache-pubsub')
    else
      withDb;
    local withGateway = if self.gateway then
      withCachePubSub.addDependsOn('gateway')
    else
      withCachePubSub;

    // Other services

    local withDeps = std.foldr(function(dep, ser) ser.addDependsOn(dep), self.deps, withGateway);

    // Secrets

    local withSecrets = std.foldr(function(sec, ser) ser.addEnvironment(sec.dst, '${' + sec.src + '?"' + sec.err + '"}'), self.secrets, withDeps);

    withSecrets + composeServiceSpecializer(name, self.devPort),
}
