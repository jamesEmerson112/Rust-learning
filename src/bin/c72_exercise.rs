// THE VAULT RUN — Chapter 5: THE VAULT
// Inventory check before the meet: enumerate EVERYTHING in the vault.
// If it's not in the scan, you never stole it.
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Intel {
    pub codename: String,
    pub value: u32,
}

pub fn full_scan(db: &sled::Db) -> anyhow::Result<Vec<Intel>> {
    // TODO: Iterate db.iter() (each item is Result<(IVec, IVec)>). Deserialize
    // each value with serde_json::from_slice into an Intel and collect them.
    // Sort by codename for a stable order.
    let _ = db;
    Ok(Vec::new())
}

fn main() -> anyhow::Result<()> {
    let db = sled::open("c72_exercise_vault")?;
    let intel = Intel { codename: "GHOSTKEY".to_string(), value: 64000 };
    db.insert("GHOSTKEY", serde_json::to_vec(&intel)?)?;
    println!("[vault] scan: {:?} (want one GHOSTKEY entry)", full_scan(&db)?);
    drop(db);
    std::fs::remove_dir_all("c72_exercise_vault").ok();
    Ok(())
}
