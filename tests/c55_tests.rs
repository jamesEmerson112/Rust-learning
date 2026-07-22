#[path = "../src/bin/c55_exercise.rs"]
#[allow(dead_code)]
mod c55_exercise;

use c55_exercise::{access_code, keystream};

#[test]
fn keygen_base_cases() {
    assert_eq!(access_code(0), 0);
    assert_eq!(access_code(1), 1);
}

#[test]
fn keygen_small_codes() {
    assert_eq!(access_code(2), 1);
    assert_eq!(access_code(7), 13);
    assert_eq!(access_code(10), 55);
}

#[test]
fn keygen_rotates_deep() {
    // the vault rotates far past u32 territory — u64 or bust
    assert_eq!(access_code(50), 12_586_269_025);
}

#[test]
fn keystream_first_eight() {
    assert_eq!(keystream(8), vec![0, 1, 1, 2, 3, 5, 8, 13]);
}
