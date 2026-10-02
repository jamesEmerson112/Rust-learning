// RUST GENERAL HOSPITAL — Pharmacy
// The reorder list: every drug running BELOW the threshold goes on tomorrow's order.
// No SQL, just filter/map over the pharmacy's own store.
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Stock {
    pub drug: String,
    pub units: u32,
}

pub fn low_stock(db: &sled::Db, below: u32) -> anyhow::Result<Vec<Stock>> {
    // TODO: Load every Stock (iterate db.iter(), from_slice each value), then
    // keep only the drugs with FEWER than `below` units. Sort by drug name.
    // A drug sitting exactly at the threshold is not low.
    let _ = (db, below);
    Ok(Vec::new())
}

pub fn low_stock_names(db: &sled::Db, below: u32) -> anyhow::Result<Vec<String>> {
    // TODO: Same filter as low_stock, then .map(|s| s.drug).collect() to
    // project just the drug names. Sort them.
    let _ = (db, below);
    Ok(Vec::new())
}

fn main() -> anyhow::Result<()> {
    let db = sled::open("c73_exercise_sled_db")?;
    let stock = Stock { drug: "Insulin".to_string(), units: 1850 };
    db.insert("Insulin", serde_json::to_vec(&stock)?)?;
    println!("[pharmacy] reorder: {:?} (want [\"Insulin\"])", low_stock_names(&db, 5000)?);
    drop(db);
    std::fs::remove_dir_all("c73_exercise_sled_db").ok();
    Ok(())
}
