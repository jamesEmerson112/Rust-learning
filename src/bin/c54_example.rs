use clap::{Parser, Subcommand};
use std::collections::HashMap;

#[derive(Parser)]
#[command(about = "Salon scheduler CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Book {
        #[arg(long)]
        technician: String,
        #[arg(long)]
        service: String,
        #[arg(long)]
        price: u32,
    },
    List,
    Revenue,
}

struct Salon {
    bookings: Vec<(String, String, u32)>,
}

impl Salon {
    fn new() -> Self {
        Self { bookings: Vec::new() }
    }

    fn book(&mut self, technician: &str, service: &str, price: u32) {
        self.bookings.push((technician.to_string(), service.to_string(), price));
    }

    fn list(&self) -> &[(String, String, u32)] {
        &self.bookings
    }

    fn revenue_by_tech(&self) -> HashMap<String, u32> {
        let mut map = HashMap::new();
        for (tech, _, price) in &self.bookings {
            *map.entry(tech.clone()).or_insert(0) += price;
        }
        map
    }
}

fn main() {
    let mut salon = Salon::new();
    salon.book("Mai", "Gel Manicure", 4500);
    salon.book("Linh", "Pedicure", 3500);
    salon.book("Mai", "Acrylic Fill", 3000);

    println!("--- Bookings ---");
    for (tech, svc, price) in salon.list() {
        println!("  {tech}: {svc} ({price} cents)");
    }

    println!("--- Revenue ---");
    for (tech, total) in &salon.revenue_by_tech() {
        println!("  {tech}: {total} cents");
    }
}
