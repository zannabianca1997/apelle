// Build a docker compose file
function() {
  withName:: function(name) self + { name: name },

  addService:: function(name, service) self + { services+: { [name]: service } },
  addVolume:: function(name, volume) self + { volumes+: { [name]: volume } },

  clearVolumes:: function() self + { volumes: {} },
}
