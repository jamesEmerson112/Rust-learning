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
