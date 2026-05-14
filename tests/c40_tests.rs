#[path = "../src/bin/c40_exercise.rs"]
#[allow(dead_code)]
mod c40_exercise;

use c40_exercise::{describe, Appointment};

#[test]
fn describe_appointment() {
    let appt = Appointment {
        client: "Trang",
        service: "Gel Manicure",
    };
    assert_eq!(describe(&appt), "Trang booked for Gel Manicure");
}

#[test]
fn describe_another() {
    let appt = Appointment {
        client: "Mai",
        service: "Pedicure",
    };
    assert_eq!(describe(&appt), "Mai booked for Pedicure");
}
