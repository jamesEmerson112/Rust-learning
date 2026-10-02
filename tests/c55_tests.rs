#[path = "../src/bin/c55_exercise.rs"]
#[allow(dead_code)]
mod c55_exercise;

use c55_exercise::{climb_ways, ways_table};

#[test]
fn base_cases() {
    assert_eq!(climb_ways(0), 1);
    assert_eq!(climb_ways(1), 1);
}

#[test]
fn small_staircases() {
    assert_eq!(climb_ways(2), 2);
    assert_eq!(climb_ways(7), 21);
    assert_eq!(climb_ways(10), 89);
}

#[test]
fn long_stairwell() {
    // far past u32 territory — u64 or bust
    assert_eq!(climb_ways(50), 20_365_011_074);
}

#[test]
fn first_eight_in_the_table() {
    assert_eq!(ways_table(8), vec![1, 1, 2, 3, 5, 8, 13, 21]);
}
