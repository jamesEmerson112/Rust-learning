use serde::{Deserialize, Serialize};

// "Querying" without SQL: load the records, then process them with the iterator
// combinators from c21-22/c45-46 — filter() selects rows, map() projects a field.
//
// RUST GENERAL HOSPITAL: the reorder list. Every drug running below the threshold
// goes on tomorrow's order — no SQL engine, just iterators over the pharmacy's own store.
#[derive(Debug, Serialize, Deserialize)]
struct Stock {
    drug: String,
    units: u32,
}

fn load_all(db: &sled::Db) -> anyhow::Result<Vec<Stock>> {
    let mut shelf = Vec::new();
    for item in db.iter() {
        let (_k, v) = item?;
        shelf.push(serde_json::from_slice::<Stock>(&v)?);
    }
    Ok(shelf)
}

fn low_stock(db: &sled::Db, below: u32) -> anyhow::Result<Vec<Stock>> {
    let mut low: Vec<Stock> = load_all(db)?
        .into_iter()
        .filter(|s| s.units < below)
        .collect();
    low.sort_by(|a, b| a.drug.cmp(&b.drug));
    Ok(low)
}

fn low_stock_names(db: &sled::Db, below: u32) -> anyhow::Result<Vec<String>> {
    let mut names: Vec<String> = load_all(db)?
        .into_iter()
        .filter(|s| s.units < below)
        .map(|s| s.drug)
        .collect();
    names.sort();
    Ok(names)
}

fn main() -> anyhow::Result<()> {
    let db = sled::open("c73_example_sled_db")?;
    for (drug, units) in [("Paracetamol", 6400u32), ("Amoxicillin", 4200), ("Insulin", 1850)] {
        let stock = Stock { drug: drug.to_string(), units };
        db.insert(drug, serde_json::to_vec(&stock)?)?;
    }
    println!("[pharmacy] reorder (below 5000): {:?}", low_stock_names(&db, 5000)?);
    for stock in low_stock(&db, 5000)? {
        println!("  {} — {} units left", stock.drug, stock.units);
    }
    drop(db);
    std::fs::remove_dir_all("c73_example_sled_db").ok();
    Ok(())
}
