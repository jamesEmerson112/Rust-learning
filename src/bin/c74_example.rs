use serde::{Deserialize, Serialize};

// Capstone: read the supplier's CSV delivery note as in c47, and store each record in sled
// with the serde encoding from c71. Then process the stock by totaling it and finding the
// best-stocked drug. anyhow wraps the csv, serde, and sled error types in one error type, so
// a single `?` handles all of them.
//
// RUST GENERAL HOSPITAL: this puts everything from the pharmacy lessons into one pipeline.
// The delivery CSV goes into the pharmacy database, and the stock report comes out of it.
#[derive(Debug, Serialize, Deserialize)]
struct Stock {
    drug: String,
    units: u32,
}

fn import_delivery(db: &sled::Db, csv_path: &str) -> anyhow::Result<u32> {
    let mut rdr = csv::Reader::from_path(csv_path)?;
    for row in rdr.deserialize() {
        let stock: Stock = row?;
        db.insert(stock.drug.as_bytes(), serde_json::to_vec(&stock)?)?;
    }
    let mut total = 0;
    for item in db.iter() {
        let (_k, v) = item?;
        total += serde_json::from_slice::<Stock>(&v)?.units;
    }
    Ok(total)
}

fn most_stocked(db: &sled::Db) -> anyhow::Result<Option<Stock>> {
    let mut best: Option<Stock> = None;
    for item in db.iter() {
        let (_k, v) = item?;
        let stock: Stock = serde_json::from_slice(&v)?;
        if best.as_ref().is_none_or(|b| stock.units > b.units) {
            best = Some(stock);
        }
    }
    Ok(best)
}

fn main() -> anyhow::Result<()> {
    std::fs::write(
        "c74_example_delivery.csv",
        "drug,units\nAmoxicillin,4200\nInsulin,1850\nParacetamol,6400\n",
    )?;
    let db = sled::open("c74_example_sled_db")?;
    let total = import_delivery(&db, "c74_example_delivery.csv")?;
    println!("[pharmacy] {} drugs imported, {total} units on hand", db.len());
    if let Some(top) = most_stocked(&db)? {
        println!("[pharmacy] best stocked: {} ({} units)", top.drug, top.units);
    }
    drop(db);
    std::fs::remove_dir_all("c74_example_sled_db").ok();
    std::fs::remove_file("c74_example_delivery.csv").ok();
    Ok(())
}
