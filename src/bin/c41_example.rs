struct Appointment<'a> {
    client: &'a str,
    service: &'a str,
}

impl<'a> Appointment<'a> {
    fn client_name(&self) -> &str {
        self.client
    }

    fn service_name(&self) -> &str {
        self.service
    }
}

fn main() {
    let appt = Appointment {
        client: "Linh",
        service: "Acrylic Full Set",
    };
    println!("Client: {}", appt.client_name());
    println!("Service: {}", appt.service_name());
}
