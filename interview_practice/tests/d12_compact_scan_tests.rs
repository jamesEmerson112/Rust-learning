//! Tests for the in-place scan compaction in `src/bin/d12_compact_scan.rs`.
//!
//! Contents:
//! - example tests: a mixed scan, a clean scan, an all-dropout scan and an empty scan
//! - model test: many pseudo-random scans checked against a filtering version

#[path = "../src/bin/d12_compact_scan.rs"]
#[allow(dead_code)]
mod d12_compact_scan;

use d12_compact_scan::compact_scan;

/// Description: Dropouts move to the end, and the valid readings keep their order.
/// Inputs: none
/// Outputs: none
#[test]
fn moves_dropouts_to_the_end_keeping_order() {
    let mut scan = [0, 1_200, 0, 0, 850, 3_100];
    assert_eq!(compact_scan(&mut scan), 3);
    assert_eq!(scan, [1_200, 850, 3_100, 0, 0, 0]);
}

/// Description: A scan with no dropouts is left exactly as it was.
/// Inputs: none
/// Outputs: none
#[test]
fn a_clean_scan_is_unchanged() {
    let mut scan = [5, 4, 3];
    assert_eq!(compact_scan(&mut scan), 3);
    assert_eq!(scan, [5, 4, 3]);
}

/// Description: A scan of only dropouts has no valid readings.
/// Inputs: none
/// Outputs: none
#[test]
fn an_all_dropout_scan_has_no_valid_readings() {
    let mut scan = [0, 0, 0];
    assert_eq!(compact_scan(&mut scan), 0);
    assert_eq!(scan, [0, 0, 0]);
}

/// Description: An empty scan has no valid readings.
/// Inputs: none
/// Outputs: none
#[test]
fn an_empty_scan_has_no_valid_readings() {
    let mut scan: [u16; 0] = [];
    assert_eq!(compact_scan(&mut scan), 0);
}

/// Description: Two hundred pseudo-random scans, full of dropouts, match a version that filters
/// into a new vector and pads it with zeros.
/// Inputs: none
/// Outputs: none
#[test]
fn matches_a_filtering_version() {
    let mut seed: u32 = 5;
    for _ in 0..200 {
        seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        let len = ((seed >> 16) % 20) as usize;
        let mut scan: Vec<u16> = (0..len)
            .map(|_| {
                seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
                ((seed >> 16) % 4) as u16
            })
            .collect();
        let mut expected: Vec<u16> = scan.iter().copied().filter(|&r| r != 0).collect();
        let valid = expected.len();
        expected.resize(scan.len(), 0);
        assert_eq!(compact_scan(&mut scan), valid);
        assert_eq!(scan, expected);
    }
}
