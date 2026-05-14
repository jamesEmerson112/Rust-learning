struct Appointment<'a> {
    client: &'a str,
    service: &'a str,
}

fn describe<'a>(appt: &Appointment<'a>) -> String {
    format!("{} booked for {}", appt.client, appt.service)
}

fn main() {
    let client = String::from("Trang");
    let service = String::from("Gel Manicure");

    let appt = Appointment {
        client: &client,
        service: &service,
    };

    println!("{}", describe(&appt));
}
