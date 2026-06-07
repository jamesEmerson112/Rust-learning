pub struct Appointment<'a> {
    pub client: &'a str,
    pub service: &'a str,
}

pub fn describe<'a>(appt: &Appointment<'a>) -> String {
    // TODO: Return "{client} booked for {service}".
    format!("{} is booked for a {}", appt.client, appt.service)
}

fn main() {
    let appt = Appointment {
        client: "Mai",
        service: "Pedicure",
    };
    println!("{}", describe(&appt));
}
