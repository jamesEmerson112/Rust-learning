//! Tests for the fault-code ranking in `src/bin/d08_top_k_faults.rs`.
//!
//! Contents:
//! - `both`: a helper that runs both versions and checks they agree
//! - ranking tests: order, ties, short logs and k of zero
//! - agreement test: both versions match on a long pseudo-random log

#[path = "../src/bin/d08_top_k_faults.rs"]
#[allow(dead_code)]
mod d08_top_k_faults;

use d08_top_k_faults::{top_k_heap, top_k_sorted};

/// Description: Runs both versions on the same log and checks that they agree.
/// Inputs:
/// - `codes`: the fault log.
/// - `k`: how many codes to return.
/// Outputs: the shared answer.
fn both(codes: &[u16], k: usize) -> Vec<u16> {
    let sorted = top_k_sorted(codes, k);
    assert_eq!(top_k_heap(codes, k), sorted);
    sorted
}

/// Description: The most frequent code comes first.
/// Inputs: none
/// Outputs: none
#[test]
fn the_most_frequent_code_comes_first() {
    assert_eq!(both(&[3, 1, 1, 2, 2, 2], 2), vec![2, 1]);
}

/// Description: Codes with equal counts are ordered by the smaller code first.
/// Inputs: none
/// Outputs: none
#[test]
fn ties_go_to_the_smaller_code() {
    assert_eq!(both(&[5, 5, 4, 4, 9], 2), vec![4, 5]);
}

/// Description: Asking for more codes than the log holds returns every distinct code.
/// Inputs: none
/// Outputs: none
#[test]
fn k_larger_than_the_log_returns_everything() {
    assert_eq!(both(&[7, 7, 8], 5), vec![7, 8]);
}

/// Description: An empty log, or k of zero, returns nothing.
/// Inputs: none
/// Outputs: none
#[test]
fn an_empty_log_or_zero_k_returns_nothing() {
    assert!(both(&[], 3).is_empty());
    assert!(both(&[1, 2, 3], 0).is_empty());
}

/// Description: Both versions agree on a 2,000-report log for several values of k.
/// Inputs: none
/// Outputs: none
#[test]
fn both_versions_agree_on_a_long_log() {
    let mut seed: u32 = 99;
    let log: Vec<u16> = (0..2_000)
        .map(|_| {
            seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
            ((seed >> 16) % 50) as u16
        })
        .collect();
    for k in [1, 3, 10, 50, 60] {
        both(&log, k);
    }
}
