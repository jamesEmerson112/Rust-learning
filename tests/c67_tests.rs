#[path = "../src/bin/c67_exercise.rs"]
#[allow(dead_code)]
mod c67_exercise;

use c67_exercise::concurrent_revenue;

#[test]
fn ten_threads_add_100_each() {
    assert_eq!(concurrent_revenue(), 1000);
}
