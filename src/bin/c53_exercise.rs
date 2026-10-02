#[allow(unused_imports)]
use clap::Parser;

// TODO: Add #[derive(Parser, Debug)] and the three #[arg(long)] fields:
//   patient: String
//   medication: String
//   dose_mg: u32        (clap turns this into the flag --dose-mg)
#[derive(Debug)]
pub struct Args {
    pub patient: String,
    pub medication: String,
    pub dose_mg: u32,
}

pub fn format_order(args: &Args) -> String {
    // TODO: Return "Order: {medication} {dose_mg} mg for {patient}"
    let _ = args;
    String::new()
}

fn main() {
    println!("Run with: --patient \"Mr. Hung\" --medication Paracetamol --dose-mg 500");
}
