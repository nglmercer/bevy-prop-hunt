def "main" [] {
  $env.RUST_LOG = {
    "lightyear": trace
    "lightyear_aeronet": debug
    "lightyear_debug": off
    "lightyear_deterministic_replication": off
    "lightyear_interpolation": off
    "lightyear_messages": debug
    "lightyear_prediction": off
    "lightyear_replication": off
    "lightyear_sync": debug
    "lightyear_transport": debug
  } | transpose k v | each {$"($in.k)=($in.v)"} | str join ",";
  cargo run
}
