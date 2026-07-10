use std::collections::HashMap;

pub struct Salon {
    bookings: Vec<(String, String, u32)>,
}

impl Salon {
    pub fn new() -> Self {
        Self { bookings: Vec::new() }
    }

    pub fn book(&mut self, technician: &str, service: &str, price: u32) {
        // TODO: Push a tuple (technician, service, price) into bookings.
        let _ = (technician, service, price);
        self.bookings.push((technician.to_string(), service.to_string(), price));
    }

    pub fn list(&self) -> &[(String, String, u32)] {
        // TODO: Return a slice of all bookings.
        &self.bookings
    }

    pub fn revenue_by_tech(&self) -> HashMap<String, u32> {
        // TODO: Sum prices per technician and return a HashMap.
        let mut list = HashMap::new();
        for (tech, _, revenue) in &self.bookings {
            *list.entry(tech.clone()).or_insert(0) += revenue;
        }
        list
    }
}

fn main() {
    let mut salon = Salon::new();
    salon.book("Mai", "Gel Manicure", 4500);
    println!("Bookings: {}", salon.list().len());
}
