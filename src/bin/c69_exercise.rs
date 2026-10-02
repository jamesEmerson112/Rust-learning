// RUST GENERAL HOSPITAL — Pharmacy
// The pharmacy keeps every stock record on disk, so nothing is lost in a power cut.
// sled: a HashMap that lives on disk.

pub fn store(db: &sled::Db, key: &str, record: &str) -> sled::Result<()> {
    // TODO: Insert key -> record into the database. Keys and values are bytes,
    // so pass record.as_bytes(). db.insert(...) returns a Result.
    let _ = (db, key, record);
    Ok(())
}

fn main() -> sled::Result<()> {
    let db = sled::open("c69_exercise_sled_db")?;
    store(&db, "stock:paracetamol", "500 mg tablets, shelf A3")?;
    println!("[pharmacy] records stored: {} (want 1)", db.len());
    drop(db);
    std::fs::remove_dir_all("c69_exercise_sled_db").ok();
    Ok(())
}
