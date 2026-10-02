#[path = "../src/bin/c56_exercise.rs"]
#[allow(dead_code)]
mod c56_exercise;

use c56_exercise::{Route, build_route, last_stop, stop_count};

#[test]
fn empty_route_has_no_stops() {
    assert_eq!(stop_count(&Route::Dock), 0);
    assert_eq!(last_stop(&Route::Dock), None);
}

#[test]
fn three_stop_route() {
    let route = build_route(&["pharmacy", "ward-3", "icu"]);
    assert_eq!(stop_count(&route), 3);
}

#[test]
fn first_stop_is_outermost() {
    let route = build_route(&["pharmacy", "ward-3"]);
    match &route {
        Route::Stop(name, _) => assert_eq!(name, "pharmacy"),
        Route::Dock => panic!("route should start at the pharmacy, not the dock"),
    }
}

#[test]
fn last_stop_is_the_icu() {
    let route = build_route(&["pharmacy", "ward-3", "icu"]);
    assert_eq!(last_stop(&route), Some("icu".to_string()));
}

#[test]
fn single_stop_route() {
    let route = build_route(&["pharmacy"]);
    assert_eq!(stop_count(&route), 1);
    assert_eq!(last_stop(&route), Some("pharmacy".to_string()));
}
