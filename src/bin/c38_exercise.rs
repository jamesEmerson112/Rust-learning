use anyhow::{bail, Context, Result};

pub fn process_booking(price_str: &str, slot_taken: bool) -> Result<String> {
    // TODO: Parse price_str as u32 using .parse().context("invalid price number")?
    // If slot_taken, bail!("slot is already taken")
    // Otherwise return Ok(format!("Booked at {} cents", price))
    let _ = (price_str, slot_taken);
    Ok(String::new())
}

fn main() {
    println!("{:?}", process_booking("4500", false));
}
