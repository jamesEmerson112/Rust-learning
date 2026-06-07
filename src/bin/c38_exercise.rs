use anyhow::{bail, Context, Result};

fn parse_price(n: &str) -> Result<u32> {
    let result: u32 = n.parse().context("Invalid price number (wrong format)")?;
    Ok(result)
}

pub fn process_booking(price_str: &str, slot_taken: bool) -> Result<String> {
    // TODO: Parse price_str as u32 using .parse().context("invalid price number")?
    // If slot_taken, bail!("slot is already taken")
    // Otherwise return Ok(format!("Booked at {} cents", price))
    let price = parse_price(price_str)?;
    if slot_taken {
        bail!("Slot is taken");
    }
    Ok(format!("Booked at {} cents", price))
}

fn main() {
    println!("{:?}", process_booking("4500", false));
    // println!("{:?}", process_booking("4500", true));
    println!("{:?}", process_booking("45as00", false));


}
