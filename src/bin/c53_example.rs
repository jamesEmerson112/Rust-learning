// clap derives a command-line parser from a struct: each field becomes a --flag, with type
// checking, --help, and error messages generated for free.
// Coming from C: instead of walking argv[] by hand (or getopt), you describe the arguments
// once as a typed struct and clap does the parsing and validation.
use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Book a salon appointment")]
struct Args {
    #[arg(long)]
    technician: String,

    #[arg(long)]
    service: String,

    #[arg(long)]
    price: u32,
}

fn main() {
    let args = Args::parse();
    println!(
        "Booked: {} with {} for {} cents",
        args.service, args.technician, args.price
    );
}
