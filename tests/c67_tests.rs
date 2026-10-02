#[path = "../src/bin/c67_exercise.rs"]
#[allow(dead_code)]
mod c67_exercise;

use c67_exercise::fluid_total;

#[test]
fn every_entry_lands_in_the_shared_total() {
    // 10 nurses × 100 ml — the chart must read 1000, every run, no races
    assert_eq!(fluid_total(), 1000);
}
