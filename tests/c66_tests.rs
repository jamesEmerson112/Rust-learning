#[path = "../src/bin/c66_exercise.rs"]
#[allow(dead_code)]
mod c66_exercise;

use c66_exercise::supply_check;

#[test]
fn three_nurses_count_the_same_list() {
    // 3 threads × (4000 + 6500 + 3500 = 14000) = 42000
    assert_eq!(supply_check(), 42000);
}
