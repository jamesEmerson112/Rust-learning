#[path = "../src/bin/c78_exercise.rs"]
#[allow(dead_code)]
mod c78_exercise;

use c78_exercise::*;

#[test]
fn busiest_window_can_be_the_last_one() {
    // The late rush (last 3 hours) is the busiest: 6 + 12 + 9 = 27 arrivals.
    let hourly = [4, 3, 6, 12, 9];
    assert_eq!(busiest_window(&hourly, 3), 27);
}

#[test]
fn busiest_window_in_the_middle() {
    let hourly = [1, 8, 9, 1, 0];
    assert_eq!(busiest_window(&hourly, 2), 17);
}

#[test]
fn width_equal_to_len_sums_everything() {
    let hourly = [1, 2, 3];
    assert_eq!(busiest_window(&hourly, 3), 6);
}
