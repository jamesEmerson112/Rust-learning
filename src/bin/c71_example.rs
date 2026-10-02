use serde::{Deserialize, Serialize};

// Store whole structs: serialize to JSON bytes with serde (c49), insert, then
// deserialize on the way back. anyhow (c38) unifies serde + sled errors so one
// `?` handles both error types.
//
// RUST GENERAL HOSPITAL: a line of text isn't enough for stock control. Each drug is a
// struct — name plus units on hand — encoded into the database and decoded back out intact.
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
