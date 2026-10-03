use serde::{Deserialize, Serialize};

// Store whole structs. Serialize each struct to JSON bytes with serde from c49, insert the
// bytes, and deserialize them on the way back out. anyhow from c38 wraps both serde errors
// and sled errors in one error type, so a single `?` handles both.
//
// RUST GENERAL HOSPITAL: a line of text is not enough for stock control. Each drug is a
// struct holding its name and the units on hand. The struct is encoded into the database
// and decoded back out intact.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Stock {
    drug: String,
    units: u32, // tablets, vials, or bags on hand
}

fn encode_stock(db: &sled::Db, stock: &Stock) -> anyhow::Result<()> {
    let bytes = serde_json::to_vec(stock)?;
    db.insert(stock.drug.as_bytes(), bytes)?;
    Ok(())
}

fn decode_stock(db: &sled::Db, drug: &str) -> anyhow::Result<Option<Stock>> {
    match db.get(drug)? {
        Some(bytes) => Ok(Some(serde_json::from_slice(&bytes)?)),
        None => Ok(None),
    }
}

fn main() -> anyhow::Result<()> {
    let db = sled::open("c71_example_sled_db")?;
    let paracetamol = Stock { drug: "Paracetamol".to_string(), units: 6400 };
    encode_stock(&db, &paracetamol)?;
    println!("[pharmacy] Paracetamol -> {:?}", decode_stock(&db, "Paracetamol")?);
    println!("[pharmacy] Unknown     -> {:?}", decode_stock(&db, "Unknown")?);
    drop(db);
    std::fs::remove_dir_all("c71_example_sled_db").ok();
    Ok(())
}
