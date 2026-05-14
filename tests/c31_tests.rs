#[path = "../src/bin/c31_exercise.rs"]
#[allow(dead_code)]
mod c31_exercise;

use c31_exercise::shared_stations;

#[test]
fn three_stations() {
    assert_eq!(shared_stations(4500), 3);
}

#[test]
fn count_independent_of_price() {
    assert_eq!(shared_stations(0), 3);
}
