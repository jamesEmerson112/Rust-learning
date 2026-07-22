#[path = "../src/bin/c56_exercise.rs"]
#[allow(dead_code)]
mod c56_exercise;

use c56_exercise::{Route, build_route, hop_count, last_node};

#[test]
fn empty_route_is_zero_hops() {
    assert_eq!(hop_count(&Route::Exit), 0);
    assert_eq!(last_node(&Route::Exit), None);
}

#[test]
fn three_hop_route() {
    let route = build_route(&["gateway", "relay-7", "aegis-core"]);
    assert_eq!(hop_count(&route), 3);
}

#[test]
fn first_node_is_outermost() {
    let route = build_route(&["gateway", "relay-7"]);
    match &route {
        Route::Hop(name, _) => assert_eq!(name, "gateway"),
        Route::Exit => panic!("route should start at the gateway, not Exit"),
    }
}

#[test]
fn deepest_hop_is_the_vault_doorstep() {
    let route = build_route(&["gateway", "relay-7", "aegis-core"]);
    assert_eq!(last_node(&route), Some("aegis-core".to_string()));
}

#[test]
fn single_hop_route() {
    let route = build_route(&["gateway"]);
    assert_eq!(hop_count(&route), 1);
    assert_eq!(last_node(&route), Some("gateway".to_string()));
}
