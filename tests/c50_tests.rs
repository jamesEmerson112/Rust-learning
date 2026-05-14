#[path = "../src/bin/c50_exercise.rs"]
#[allow(dead_code)]
mod c50_exercise;

use c50_exercise::check_availability;

#[tokio::test]
async fn taken_slot() {
    assert!(!check_availability("10:00").await);
}

#[tokio::test]
async fn free_slot() {
    assert!(check_availability("11:00").await);
}

#[tokio::test]
async fn another_free_slot() {
    assert!(check_availability("14:30").await);
}
