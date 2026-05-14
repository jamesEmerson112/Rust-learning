#[path = "../src/bin/c51_exercise.rs"]
#[allow(dead_code)]
mod c51_exercise;

use c51_exercise::book_two;

#[tokio::test]
async fn both_booked() {
    let (a, b) = book_two().await;
    assert_eq!(a, "Mai booked at 10:00");
    assert_eq!(b, "Linh booked at 10:30");
}
