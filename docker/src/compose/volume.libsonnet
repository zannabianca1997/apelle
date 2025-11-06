// Build a docker compose volume
function() {
  withName:: function(name) self + { name: name },
}
