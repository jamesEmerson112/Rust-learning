use clap::Parser;

// TODO: Add #[derive(Parser, Debug)] and the three #[arg(long)] fields:
//   technician: String
//   service: String
//   price: u32
#[derive(Debug)]
pub struct Args {
    pub technician: String,
    pub service: String,
    pub price: u32,
}

pub fn format_booking(args: &Args) -> String {
    // TODO: Return "Booked: {service} with {technician} for {price} cents"
    let _ = args;
    String::new()
}

fn main() {
    println!("Run with: --technician Mai --service \"Gel Manicure\" --price 4500");
}
