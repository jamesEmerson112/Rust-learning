#[path = "../src/bin/c68_exercise.rs"]
#[allow(dead_code)]
mod c68_exercise;

use c68_exercise::read_write_schedule;

#[test]
fn readers_see_written_appointment() {
    assert_eq!(read_write_schedule(), 2);
}
