#[path = "../src/bin/c41_exercise.rs"]
#[allow(dead_code)]
mod c41_exercise;

use c41_exercise::Appointment;

#[test]
fn client_name_returned() {
    let appt = Appointment {
        client: "Linh",
        service: "Acrylic Full Set",
    };
    assert_eq!(appt.client_name(), "Linh");
}

#[test]
fn service_name_returned() {
    let appt = Appointment {
        client: "Trang",
        service: "Gel Manicure",
    };
    assert_eq!(appt.service_name(), "Gel Manicure");
}
