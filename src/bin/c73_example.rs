use serde::{Deserialize, Serialize};

// "Querying" without SQL: load the records, then process them with the iterator
// combinators from c21-22/c45-46 — filter() selects rows, map() projects a field.
//
// THE VAULT RUN: the fence only moves premium goods. Shortlist the intel worth
// more than the threshold — no SQL engine, just iterators over your own store.
#[derive(Debug, Serialize, Deserialize)]
struct Intel {
    codename: String,
    value: u32,
}

fn load_all(db: &sled::Db) -> anyhow::Result<Vec<Intel>> {
    let mut haul = Vec::new();
    for item in db.iter() {
        let (_k, v) = item?;
        haul.push(serde_json::from_slice::<Intel>(&v)?);
    }
    Ok(haul)
}

fn shortlist(db: &sled::Db, min_value: u32) -> anyhow::Result<Vec<Intel>> {
    let mut premium: Vec<Intel> = load_all(db)?
        .into_iter()
        .filter(|i| i.value >= min_value)
        .collect();
    premium.sort_by(|a, b| a.codename.cmp(&b.codename));
    Ok(premium)
}

fn shortlist_codenames(db: &sled::Db, min_value: u32) -> anyhow::Result<Vec<String>> {
    let mut names: Vec<String> = load_all(db)?
        .into_iter()
        .filter(|i| i.value >= min_value)
        .map(|i| i.codename)
        .collect();
    names.sort();
    Ok(names)
}

fn main() -> anyhow::Result<()> {
    let db = sled::open("c73_example_vault")?;
    for (codename, value) in [("GHOSTKEY", 64000u32), ("BLACKOUT", 42000), ("EXEC-DIRT", 18500)] {
        let intel = Intel { codename: codename.to_string(), value };
        db.insert(codename, serde_json::to_vec(&intel)?)?;
    }
    println!("[fence] premium codenames (>= 40000): {:?}", shortlist_codenames(&db, 40000)?);
    for intel in shortlist(&db, 40000)? {
        println!("  {} — {} creds", intel.codename, intel.value);
    }
    drop(db);
    std::fs::remove_dir_all("c73_example_vault").ok();
    Ok(())
}
