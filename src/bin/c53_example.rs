// clap derives a command-line parser from a struct: each field becomes a --flag, with type
// checking, --help, and error messages generated for free.
// Coming from C: instead of walking argv[] by hand (or getopt), you describe the arguments
// once as a typed struct and clap does the parsing and validation.
//
// RUST GENERAL HOSPITAL: a doctor writes a medication order from the ward terminal.
// Run: cargo run --bin c53_example -- --patient "Mr. Hung" --medication Paracetamol --dose-mg 500
use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Write a medication order")]
struct Args {
    #[arg(long)]
    patient: String,

    #[arg(long)]
    medication: String,

    #[arg(long)]
    dose_mg: u32,
}

fn main() {
    let args = Args::parse();
    println!(
        "Order: {} {} mg for {}",
        args.medication, args.dose_mg, args.patient
    );
}
