// Build a docker compose build section
function() {
  withContext:: function(context) self + { context: context },
  withDockerfile:: function(dockerfile) self + { dockerfile: dockerfile },
  withTarget:: function(target) self + { target: target },
  addArg:: function(arg, value) self + { args+: { [arg]: value } },
}
