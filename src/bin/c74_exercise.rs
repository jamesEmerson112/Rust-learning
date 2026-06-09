use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub price: u32,
}

pub fn import_and_total(db: &sled::Db, csv_path: &str) -> anyhow::Result<u32> {
    // TODO: Read the CSV at csv_path with csv::Reader::from_path, deserialize each
    // row into a Service, and db.insert it (serde_json::to_vec). Then iterate the
    // store and sum the prices. anyhow's `?` unifies csv + serde + sled errors.
    let _ = (db, csv_path);
    Ok(0)
}

fn main() -> anyhow::Result<()> {
    std::fs::write("c74_exercise.csv", "name,price\nGel,4500\nPedicure,3500\n")?;
    let db = sled::open("c74_exercise_sled_db")?;
    println!("total: {}", import_and_total(&db, "c74_exercise.csv")?);
    drop(db);
    std::fs::remove_dir_all("c74_exercise_sled_db").ok();
    std::fs::remove_file("c74_exercise.csv").ok();
    Ok(())
}
