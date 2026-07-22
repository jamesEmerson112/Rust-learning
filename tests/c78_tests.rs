#[path = "../src/bin/c78_exercise.rs"]
#[allow(dead_code)]
mod c78_exercise;

use c78_exercise::*;

#[test]
fn busiest_window_can_be_the_last_one() {
    // The closing rush (last 3 hours) is the busiest: 6000 + 12000 + 9000 = 27000.
    let hourly = [4500, 3500, 6000, 12000, 9000];
    assert_eq!(busiest_window(&hourly, 3), 27000);
}

#[test]
fn busiest_window_in_the_middle() {
    let hourly = [1000, 8000, 9000, 1000, 500];
    assert_eq!(busiest_window(&hourly, 2), 17000);
}

#[test]
fn width_equal_to_len_sums_everything() {
    let hourly = [100, 200, 300];
    assert_eq!(busiest_window(&hourly, 3), 600);
}
