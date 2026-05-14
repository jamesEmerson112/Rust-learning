#[path = "../src/bin/c33_exercise.rs"]
#[allow(dead_code)]
mod c33_exercise;

use c33_exercise::ServiceLog;

#[test]
fn average_for_known_technician() {
    let mut log = ServiceLog::new();
    log.add_service("Mai", 4000);
    log.add_service("Mai", 6000);

    let avg = log.average_revenue("Mai").expect("expected average for Mai");
    assert!((avg - 5000.0).abs() < 1e-9);
}

#[test]
fn average_for_unknown_technician_is_none() {
    let log = ServiceLog::new();
    assert_eq!(log.average_revenue("Unknown"), None);
}

#[test]
fn average_precision() {
    let mut log = ServiceLog::new();
    log.add_service("Linh", 4500);
    log.add_service("Linh", 5500);

    let avg = log.average_revenue("Linh").expect("expected average for Linh");
    let rounded = (avg * 100.0).round() / 100.0;
    assert_eq!(rounded, 5000.0);
}
