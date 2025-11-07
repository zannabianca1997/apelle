// Common compose before being specialized for prod or dev

local compose = import 'compose/index.libsonnet';
local deploy = import 'deploy.json';
local service = import 'service.libsonnet';

local base = compose.file()
             .withName('apelle');

local withServices = std.foldr(
  function(ser, file)
    file.addService(
      ser.key,
      (service + std.prune(ser.value)).composeService(ser.key)
    ),
  std.objectKeysValues(deploy.services),
  base
);

local withDb = (import 'special-services/db.libsonnet')(withServices);
local withFront = (import 'special-services/front.libsonnet')(withDb);
local withGateway = (import 'special-services/gateway.libsonnet')(withFront);
local withCachePubsub = (import 'special-services/cache-pubsub.libsonnet')(withGateway);

withCachePubsub {
  specializeFor:: function(environment)
    local withSpecializedServices = self + {
      services: {
        [s.key]: s.value.specializeFor(environment)
        for s in std.objectKeysValues($.services)
      },
    };

    // Add certbot volume
    local withCertVolume = if environment == 'prod' then
      withSpecializedServices.addVolume(
        'certbot-www',
        compose.volume().withName('${COMPOSE_PROJECT_NAME}-certbot-www')
      ).addVolume(
        'certbot-conf',
        compose.volume().withName('${COMPOSE_PROJECT_NAME}-certbot-conf')
      )
    else withSpecializedServices;

    local withoutVolumesInBuild = if environment == 'build' then
      withCertVolume.clearVolumes()
    else withCertVolume;

    local withName = withoutVolumesInBuild.withName(withoutVolumesInBuild.name + '-' + if environment == 'dev' then 'dev' else 'prod');

    std.prune(withName),
}
