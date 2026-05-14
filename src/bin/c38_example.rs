use anyhow::{bail, Context, Result};

fn parse_price(s: &str) -> Result<u32> {
    let n: u32 = s.parse().context("invalid price number")?;
    Ok(n)
}

fn process_booking(price_str: &str, slot_taken: bool) -> Result<String> {
    let price = parse_price(price_str)?;
    if slot_taken {
        bail!("slot is already taken");
    }
    Ok(format!("Booked at {} cents", price))
}

fn main() {
    println!("{:?}", process_booking("4500", false));
    println!("{:?}", process_booking("abc", false));
    println!("{:?}", process_booking("4500", true));
}
