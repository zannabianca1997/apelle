// Build a docker compose service
function() {
  withImage:: function(image) self + { image: image },
  withContainerName:: function(name) self + { container_name: name },
  withRestart:: function(policy) self + { restart: policy },
  withBuild:: function(build) self + { build: build },
  withCommand:: function(command) self + { command: command },

  addDependsOn:: function(dependency, condition='service_started') self + { depends_on+: { [dependency]: { condition: condition } } },
  addEnvironment:: function(name, value) self + { environment+: { [name]: value } },
  addVolume:: function(volume) self + { volumes+: [volume] },
  addPort:: function(port) self + { ports+: [port] },
}
