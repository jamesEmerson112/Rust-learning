//! Tests for the waypoint tree and its iterator in `src/bin/d09_waypoint_iterator.rs`.
//!
//! Contents:
//! - tree tests: insert, duplicates and lookups
//! - iterator tests: order, `has_next`, a lopsided tree and iterator adaptors

#[path = "../src/bin/d09_waypoint_iterator.rs"]
#[allow(dead_code)]
mod d09_waypoint_iterator;

use d09_waypoint_iterator::Waypoints;

/// Description: An empty tree yields nothing and reports no next waypoint.
/// Inputs: none
/// Outputs: none
#[test]
fn an_empty_tree_yields_nothing() {
    let route = Waypoints::new();
    let mut walk = route.iter();
    assert!(!walk.has_next());
    assert_eq!(walk.next(), None);
}

/// Description: Waypoints inserted out of order come back from nearest to farthest.
/// Inputs: none
/// Outputs: none
#[test]
fn walks_in_increasing_order() {
    let mut route = Waypoints::new();
    for meters in [50, 20, 80, 10, 30, 70, 90] {
        route.insert(meters);
    }
    assert_eq!(route.iter().collect::<Vec<_>>(), vec![10, 20, 30, 50, 70, 80, 90]);
}

/// Description: Inserting a waypoint twice keeps one copy.
/// Inputs: none
/// Outputs: none
#[test]
fn insert_rejects_duplicates() {
    let mut route = Waypoints::new();
    assert!(route.insert(5));
    assert!(!route.insert(5));
    assert_eq!(route.iter().count(), 1);
}

/// Description: The tree finds inserted waypoints and nothing else.
/// Inputs: none
/// Outputs: none
#[test]
fn contains_finds_only_inserted_waypoints() {
    let mut route = Waypoints::new();
    for meters in [40, 10, 60] {
        route.insert(meters);
    }
    assert!(route.contains(10));
    assert!(route.contains(60));
    assert!(!route.contains(50));
}

/// Description: `has_next` stays true until the last waypoint has been yielded.
/// Inputs: none
/// Outputs: none
#[test]
fn has_next_tracks_progress() {
    let mut route = Waypoints::new();
    route.insert(2);
    route.insert(1);
    let mut walk = route.iter();
    assert!(walk.has_next());
    assert_eq!(walk.next(), Some(1));
    assert!(walk.has_next());
    assert_eq!(walk.next(), Some(2));
    assert!(!walk.has_next());
    assert_eq!(walk.next(), None);
}

/// Description: Inserting 1,000 waypoints in order builds a lopsided tree, and both the insert
/// loop and the walk still work.
/// Inputs: none
/// Outputs: none
#[test]
fn a_lopsided_tree_still_walks_correctly() {
    let mut route = Waypoints::new();
    for meters in 0..1_000 {
        route.insert(meters);
    }
    assert_eq!(route.iter().collect::<Vec<_>>(), (0..1_000).collect::<Vec<_>>());
}

/// Description: Because the walk is a real `Iterator`, adaptors such as `skip_while` and
/// `take_while` select a stretch of the route.
/// Inputs: none
/// Outputs: none
#[test]
fn iterator_adaptors_work_on_the_walk() {
    let mut route = Waypoints::new();
    for meters in [50, 20, 80, 10, 30, 70, 90] {
        route.insert(meters);
    }
    let stretch: Vec<u32> =
        route.iter().skip_while(|&m| m < 25).take_while(|&m| m <= 75).collect();
    assert_eq!(stretch, vec![30, 50, 70]);
}
