#[path = "../src/bin/c79_exercise.rs"]
#[allow(dead_code)]
mod c79_exercise;

use c79_exercise::*;

#[test]
fn adds_distinct_bookings() {
    let sched = Schedule::new();
    assert!(sched.add_if_absent("Mr. Hung - X-ray"));
    assert!(sched.add_if_absent("Mrs. Lan - MRI"));
    assert_eq!(sched.len(), 2);
}

#[test]
fn rejects_a_duplicate_booking() {
    let sched = Schedule::new();
    assert!(sched.add_if_absent("Mr. Hung - X-ray"));
    assert!(!sched.add_if_absent("Mr. Hung - X-ray"));
    assert_eq!(sched.len(), 1);
}
