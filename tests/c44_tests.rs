#[path = "../src/bin/c44_exercise.rs"]
#[allow(dead_code)]
mod c44_exercise;

use c44_exercise::shared_book_count;

#[test]
fn two_stations_two_appointments() {
    assert_eq!(shared_book_count(), 2);
}
