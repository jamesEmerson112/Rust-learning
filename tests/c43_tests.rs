#[path = "../src/bin/c43_exercise.rs"]
#[allow(dead_code)]
mod c43_exercise;

use c43_exercise::Schedule;

#[test]
fn starts_empty() {
    let sched = Schedule::new();
    assert_eq!(sched.count(), 0);
}

#[test]
fn add_appointments() {
    let sched = Schedule::new();
    sched.add("Mai - Gel Manicure");
    sched.add("Linh - Pedicure");
    assert_eq!(sched.count(), 2);
}

#[test]
fn through_shared_ref() {
    let sched = Schedule::new();
    sched.add("Trang - Acrylic");
    sched.add("Mai - Fill");
    sched.add("Linh - Gel");
    assert_eq!(sched.count(), 3);
}
