// RUST GENERAL HOSPITAL — Pharmacy
// The monthly stock take: list EVERYTHING in the pharmacy database.
// If it isn't in the scan, it isn't on the shelf.
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Stock {
    pub drug: String,
    pub units: u32,
}

pub fn full_inventory(db: &sled::Db) -> anyhow::Result<Vec<Stock>> {
    // TODO: Iterate db.iter() (each item is Result<(IVec, IVec)>). Deserialize
    // each value with serde_json::from_slice into a Stock and collect them.
    // Sort by drug name for a stable order.
    let _ = db;
    Ok(Vec::new())
}

fn main() -> anyhow::Result<()> {
    let db = sled::open("c72_exercise_sled_db")?;
    let stock = Stock { drug: "Paracetamol".to_string(), units: 6400 };
    db.insert("Paracetamol", serde_json::to_vec(&stock)?)?;
    println!("[pharmacy] stock take: {:?} (want one Paracetamol entry)", full_inventory(&db)?);
    drop(db);
    std::fs::remove_dir_all("c72_exercise_sled_db").ok();
    Ok(())
}
