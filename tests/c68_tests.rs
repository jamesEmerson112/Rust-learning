#[path = "../src/bin/c68_exercise.rs"]
#[allow(dead_code)]
mod c68_exercise;

use c68_exercise::alert_board_count;

#[test]
fn lookouts_see_the_spotters_update() {
    assert_eq!(alert_board_count(), 2);
}
