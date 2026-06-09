use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub price: u32,
}

pub fn all_services(db: &sled::Db) -> anyhow::Result<Vec<Service>> {
    // TODO: Iterate db.iter() (each item is Result<(IVec, IVec)>). Deserialize
    // each value with serde_json::from_slice into a Service and collect them.
    // Sort by name for a stable order.
    let _ = db;
    Ok(Vec::new())
}

fn main() -> anyhow::Result<()> {
    let db = sled::open("c72_exercise_sled_db")?;
    db.insert("Gel", serde_json::to_vec(&Service { name: "Gel".to_string(), price: 4500 })?)?;
    println!("{:?}", all_services(&db)?);
    drop(db);
    std::fs::remove_dir_all("c72_exercise_sled_db").ok();
    Ok(())
}
