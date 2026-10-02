// RUST GENERAL HOSPITAL — Pharmacy
// A line of text isn't enough for stock control. Store each drug as a struct — name
// plus units on hand — encoded into the database as JSON bytes and decoded back out intact.
// (The c74 capstone will import THIS file and reuse your codec. Build it well.)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stock {
    pub drug: String,
    pub units: u32, // tablets, vials, or bags on hand
}

pub fn encode_stock(db: &sled::Db, stock: &Stock) -> anyhow::Result<()> {
    // TODO: Serialize the stock record to JSON bytes with serde_json::to_vec, then
    // db.insert it under stock.drug.as_bytes(). anyhow's `?` unifies the
    // serde + sled error types.
    let _ = (db, stock);
    Ok(())
}

pub fn decode_stock(db: &sled::Db, drug: &str) -> anyhow::Result<Option<Stock>> {
    // TODO: db.get(drug)? gives Option<IVec>; deserialize the bytes with
    // serde_json::from_slice into a Stock. Return None when the key is absent.
    let _ = (db, drug);
    Ok(None)
}

fn main() -> anyhow::Result<()> {
    let db = sled::open("c71_exercise_sled_db")?;
    let paracetamol = Stock { drug: "Paracetamol".to_string(), units: 6400 };
    encode_stock(&db, &paracetamol)?;
    println!("[pharmacy] Paracetamol -> {:?} (want Some(..))", decode_stock(&db, "Paracetamol")?);
    drop(db);
    std::fs::remove_dir_all("c71_exercise_sled_db").ok();
    Ok(())
}
