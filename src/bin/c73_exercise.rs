use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub price: u32,
}

pub fn services_over(db: &sled::Db, min_price: u32) -> anyhow::Result<Vec<Service>> {
    // TODO: Load every Service (iterate db.iter(), from_slice each value), then
    // .filter(|s| s.price >= min_price).collect(). Sort by name.
    let _ = (db, min_price);
    Ok(Vec::new())
}

pub fn names_over(db: &sled::Db, min_price: u32) -> anyhow::Result<Vec<String>> {
    // TODO: Same filter as services_over, then .map(|s| s.name).collect() to
    // project just the names. Sort them.
    let _ = (db, min_price);
    Ok(Vec::new())
}

fn main() -> anyhow::Result<()> {
    let db = sled::open("c73_exercise_sled_db")?;
    db.insert("Gel", serde_json::to_vec(&Service { name: "Gel".to_string(), price: 4500 })?)?;
    println!("{:?}", names_over(&db, 4000)?);
    drop(db);
    std::fs::remove_dir_all("c73_exercise_sled_db").ok();
    Ok(())
}
