#[path = "../src/bin/c60_exercise.rs"]
#[allow(dead_code)]
mod c60_exercise;

use c60_exercise::{early_stop, stop_order};

#[test]
fn pumps_stop_in_reverse_order() {
    assert_eq!(
        stop_order(),
        vec!["bed-2 pump stopped".to_string(), "bed-1 pump stopped".to_string()]
    );
}

#[test]
fn blocked_line_stops_first() {
    assert_eq!(
        early_stop(),
        vec!["bed-1 pump stopped".to_string(), "bed-2 pump stopped".to_string()]
    );
}
