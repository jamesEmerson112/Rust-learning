// RUST GENERAL HOSPITAL — Pharmacy
// For the monthly stock take, list everything in the pharmacy database.
// Anything missing from the scan counts as missing from the shelf.
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Stock {
    pub drug: String,
    pub units: u32,
}

pub fn full_inventory(db: &sled::Db) -> anyhow::Result<Vec<Stock>> {
    // TODO: Loop over db.iter(), where each item is a Result<(IVec, IVec)>. Deserialize
    // each value into a Stock with serde_json::from_slice, and collect them.
    // Sort by drug name so the order is the same on every run.
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
