use serde::{Deserialize, Serialize};

// Scan the whole store. db.iter() yields items of type Result<(IVec, IVec)>, where each
// pair is a key and a value. Take the value, decode it, and collect the results. Sort them
// so the output comes out in the same order on every run.
//
// RUST GENERAL HOSPITAL: in the monthly stock take, the pharmacy lists everything it holds.
// Anything missing from the scan counts as missing from the shelf.
#[derive(Debug, Serialize, Deserialize)]
struct Stock {
    drug: String,
    units: u32,
}

fn full_inventory(db: &sled::Db) -> anyhow::Result<Vec<Stock>> {
    let mut shelf = Vec::new();
    for item in db.iter() {
        let (_key, value) = item?;
        shelf.push(serde_json::from_slice::<Stock>(&value)?);
    }
    shelf.sort_by(|a, b| a.drug.cmp(&b.drug));
    Ok(shelf)
}

fn main() -> anyhow::Result<()> {
    let db = sled::open("c72_example_sled_db")?;
    for (drug, units) in [("Paracetamol", 6400u32), ("Amoxicillin", 4200), ("Insulin", 1850)] {
        let stock = Stock { drug: drug.to_string(), units };
        db.insert(drug, serde_json::to_vec(&stock)?)?;
    }
    println!("[pharmacy] full stock take:");
    for stock in full_inventory(&db)? {
        println!("  {} — {} units", stock.drug, stock.units);
    }
    drop(db);
    std::fs::remove_dir_all("c72_example_sled_db").ok();
    Ok(())
}
