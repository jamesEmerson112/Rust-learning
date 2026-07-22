use serde::{Deserialize, Serialize};

// Capstone: read the exfiltrated CSV dump (c47 skill), store each record in sled via the
// serde codec (c71 pattern), then PROCESS the haul — total it and find the crown jewel.
// anyhow unifies the csv + serde + sled error types so a single `?` style handles them all.
//
// THE VAULT RUN — FINALE. Everything this chapter built, assembled into one pipeline:
// CSV dump -> your datavault -> the fence's bottom line.
#[derive(Debug, Serialize, Deserialize)]
struct Intel {
    codename: String,
    value: u32,
}

fn import_dump(db: &sled::Db, csv_path: &str) -> anyhow::Result<u32> {
    let mut rdr = csv::Reader::from_path(csv_path)?;
    for row in rdr.deserialize() {
        let intel: Intel = row?;
        db.insert(intel.codename.as_bytes(), serde_json::to_vec(&intel)?)?;
    }
    let mut total = 0;
    for item in db.iter() {
        let (_k, v) = item?;
        total += serde_json::from_slice::<Intel>(&v)?.value;
    }
    Ok(total)
}

fn crown_jewel(db: &sled::Db) -> anyhow::Result<Option<Intel>> {
    let mut best: Option<Intel> = None;
    for item in db.iter() {
        let (_k, v) = item?;
        let intel: Intel = serde_json::from_slice(&v)?;
        if best.as_ref().is_none_or(|b| intel.value > b.value) {
            best = Some(intel);
        }
    }
    Ok(best)
}

fn main() -> anyhow::Result<()> {
    std::fs::write(
        "c74_example_dump.csv",
        "codename,value\nBLACKOUT,42000\nEXEC-DIRT,18500\nGHOSTKEY,64000\n",
    )?;
    let db = sled::open("c74_example_vault")?;
    let haul = import_dump(&db, "c74_example_dump.csv")?;
    println!("[finale] {} records imported, haul: {haul} creds", db.len());
    if let Some(jewel) = crown_jewel(&db)? {
        println!("[finale] crown jewel: {} ({} creds)", jewel.codename, jewel.value);
    }
    drop(db);
    std::fs::remove_dir_all("c74_example_vault").ok();
    std::fs::remove_file("c74_example_dump.csv").ok();
    Ok(())
}
