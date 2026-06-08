#[path = "../src/bin/c66_exercise.rs"]
#[allow(dead_code)]
mod c66_exercise;

use c66_exercise::shared_across_threads;

#[test]
fn three_threads_each_sum() {
    // 3 threads × (4500 + 6000 + 3500 = 14000) = 42000
    assert_eq!(shared_across_threads(), 42000);
}
