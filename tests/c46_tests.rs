#[path = "../src/bin/c46_exercise.rs"]
#[allow(dead_code)]
mod c46_exercise;

use c46_exercise::{gel_prices, WalkInQueue};

#[test]
fn filters_gel_only() {
    let queue = WalkInQueue::new(vec![
        "Mai:Gel:4500".to_string(),
        "Linh:Pedicure:3500".to_string(),
        "Trang:Gel:5000".to_string(),
    ]);
    assert_eq!(gel_prices(queue), vec![4500, 5000]);
}

#[test]
fn no_gel_services() {
    let queue = WalkInQueue::new(vec![
        "Mai:Pedicure:3500".to_string(),
    ]);
    assert_eq!(gel_prices(queue), Vec::<u32>::new());
}

#[test]
fn empty_queue() {
    let queue = WalkInQueue::new(vec![]);
    assert_eq!(gel_prices(queue), Vec::<u32>::new());
}
