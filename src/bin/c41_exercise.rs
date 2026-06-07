pub struct Appointment<'a> {
    pub client: &'a str,
    pub service: &'a str,
}

impl<'a> Appointment<'a> {
    pub fn client_name(&self) -> &str {
        // TODO: Return the client's name. Rust elides the lifetime here.
        self.client
    }

    pub fn service_name(&self) -> &str {
        // TODO: Return the service name.
        self.service
    }
}

fn main() {
    let appt = Appointment {
        client: "Mai",
        service: "Dip Powder",
    };
    println!("{}: {}", appt.client_name(), appt.service_name());
}
