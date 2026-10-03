// RUST GENERAL HOSPITAL — Pharmacy
// A line of text is not enough for stock control. Store each drug as a struct holding its
// name and the units on hand. Encode it into the database as JSON bytes, and decode it back
// out intact.
// The c74 capstone imports this file and reuses your encode and decode functions, so they
// need to work correctly.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stock {
    pub drug: String,
    pub units: u32, // tablets, vials, or bags on hand
}

pub fn encode_stock(db: &sled::Db, stock: &Stock) -> anyhow::Result<()> {
    // TODO: Serialize the stock record to JSON bytes with serde_json::to_vec, then
    // db.insert it under the key stock.drug.as_bytes(). With anyhow, a single `?` works
    // for both the serde error type and the sled error type.
    let _ = (db, stock);
    Ok(())
}

pub fn decode_stock(db: &sled::Db, drug: &str) -> anyhow::Result<Option<Stock>> {
    // TODO: db.get(drug)? gives an Option<IVec>. Deserialize the bytes into a Stock with
    // serde_json::from_slice. Return None when the key is absent.
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
