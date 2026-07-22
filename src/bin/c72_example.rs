use serde::{Deserialize, Serialize};

// Scan the whole store. db.iter() yields Result<(IVec, IVec)> pairs (key, value);
// take the value, decode it, collect. Sort for a deterministic order.
//
// THE VAULT RUN: inventory check before the meet — enumerate EVERYTHING you're
// holding. If it's not in the scan, you never stole it.
#[derive(Debug, Serialize, Deserialize)]
struct Intel {
    codename: String,
    value: u32,
}

fn full_scan(db: &sled::Db) -> anyhow::Result<Vec<Intel>> {
    let mut haul = Vec::new();
    for item in db.iter() {
        let (_key, value) = item?;
        haul.push(serde_json::from_slice::<Intel>(&value)?);
    }
    haul.sort_by(|a, b| a.codename.cmp(&b.codename));
    Ok(haul)
}

fn main() -> anyhow::Result<()> {
    let db = sled::open("c72_example_vault")?;
    for (codename, value) in [("GHOSTKEY", 64000u32), ("BLACKOUT", 42000), ("EXEC-DIRT", 18500)] {
        let intel = Intel { codename: codename.to_string(), value };
        db.insert(codename, serde_json::to_vec(&intel)?)?;
    }
    println!("[vault] full inventory scan:");
    for intel in full_scan(&db)? {
        println!("  {} — {} creds", intel.codename, intel.value);
    }
    drop(db);
    std::fs::remove_dir_all("c72_example_vault").ok();
    Ok(())
}
