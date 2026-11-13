//! Tests for the obstacle cluster search in `src/bin/d13_obstacle_clusters.rs`.
//!
//! Contents:
//! - counting tests: separate clusters, corner contact and free grids
//! - shape tests: a snake, a ring and clusters on the grid edge

#[path = "../src/bin/d13_obstacle_clusters.rs"]
#[allow(dead_code)]
mod d13_obstacle_clusters;

use d13_obstacle_clusters::cluster_sizes;

/// Description: A grid with four clusters reports their sizes, largest first.
/// Inputs: none
/// Outputs: none
#[test]
fn counts_separate_clusters() {
    let grid = ["##..#", "#...#", "..#..", "....#"];
    assert_eq!(cluster_sizes(&grid), vec![3, 2, 1, 1]);
}

/// Description: Two obstacles that touch only at a corner are two clusters.
/// Inputs: none
/// Outputs: none
#[test]
fn corner_contact_does_not_join_clusters() {
    assert_eq!(cluster_sizes(&["#.", ".#"]), vec![1, 1]);
}

/// Description: A grid with no obstacles, or no rows at all, has no clusters.
/// Inputs: none
/// Outputs: none
#[test]
fn a_free_grid_has_no_clusters() {
    assert!(cluster_sizes(&["...", "..."]).is_empty());
    assert!(cluster_sizes(&[]).is_empty());
}

/// Description: A winding wall of obstacles is one cluster, however it bends.
/// Inputs: none
/// Outputs: none
#[test]
fn a_winding_wall_is_one_cluster() {
    assert_eq!(cluster_sizes(&["###", "..#", "###"]), vec![7]);
}

/// Description: A ring around a free cell is one cluster of eight cells.
/// Inputs: none
/// Outputs: none
#[test]
fn a_ring_is_one_cluster() {
    assert_eq!(cluster_sizes(&["###", "#.#", "###"]), vec![8]);
}

/// Description: Obstacles at both ends of a row stay separate, because a neighbour search on the
/// edge does not wrap around.
/// Inputs: none
/// Outputs: none
#[test]
fn edge_cells_do_not_wrap_around() {
    assert_eq!(cluster_sizes(&["#..#"]), vec![1, 1]);
}
