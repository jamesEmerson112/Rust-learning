#[path = "../src/bin/c79_exercise.rs"]
#[allow(dead_code)]
mod c79_exercise;

use c79_exercise::*;

#[test]
fn adds_distinct_bookings() {
    let sched = Schedule::new();
    assert!(sched.add_if_absent("Mai - Gel Manicure"));
    assert!(sched.add_if_absent("Linh - Pedicure"));
    assert_eq!(sched.len(), 2);
}

#[test]
fn rejects_a_duplicate_booking() {
    let sched = Schedule::new();
    assert!(sched.add_if_absent("Mai - Gel Manicure"));
    assert!(!sched.add_if_absent("Mai - Gel Manicure"));
    assert_eq!(sched.len(), 1);
}
