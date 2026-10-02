use serde::{Deserialize, Serialize};

// Scan the whole store. db.iter() yields Result<(IVec, IVec)> pairs (key, value);
// take the value, decode it, collect. Sort for a deterministic order.
//
// RUST GENERAL HOSPITAL: the monthly stock take — list EVERYTHING the pharmacy holds.
// If it isn't in the scan, it isn't on the shelf.
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
