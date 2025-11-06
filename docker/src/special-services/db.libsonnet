// PostgreSQL database service

local compose = import '../compose/index.libsonnet';

local dbService = compose.service()
                  .withImage('postgres:16-alpine')
                  .withContainerName('${COMPOSE_PROJECT_NAME}-db')
                  .withRestart('always')
                  .addEnvironment('POSTGRES_DB', '${POSTGRES_DB:-apelle}')
                  .addEnvironment('POSTGRES_USER', '${POSTGRES_USER:-apelle}')
                  .addVolume('db-data:/var/lib/postgresql/data') + {
  specializeFor:: function(enviroment)
    if enviroment == 'dev' then
      self
      .addEnvironment('POSTGRES_PASSWORD', '${POSTGRES_PASSWORD:-apelle}')
      .addPort('5432:5432')
    else if enviroment == 'prod' then
      self.addEnvironment('POSTGRES_PASSWORD', '${POSTGRES_PASSWORD?-"Please set database password"}')
    else
      // build enviroment, no database
      null,
};

local migratorService = (import '../service.libsonnet')
                        .composeService('migrator')
                        // Adding db manually so it does not add migrator
                        .addDependsOn('db')
                        .withRestart('no');

// Add the service to a docker compose
function(file)
  file
  .addService('db', dbService)
  .addService('migrator', migratorService)
  .addVolume(
    'db-data',
    compose.volume().withName('${COMPOSE_PROJECT_NAME}-db-data')
  )
