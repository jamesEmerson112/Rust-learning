// sled is a pure-Rust embedded key-value database — think a HashMap that lives on disk and
// survives restarts. No SQL, no schema, no server: open a directory, then insert/get byte
// keys and values. Coming from C: a B-tree-backed store you link into the binary, like
// Berkeley DB — no daemon to talk to.
//
// RUST GENERAL HOSPITAL: the pharmacy keeps its stock records in a local database, so the
// records survive a power cut or a restart.
fn store(db: &sled::Db, key: &str, record: &str) -> sled::Result<()> {
    db.insert(key, record.as_bytes())?; // keys and values are bytes
    Ok(())
}

fn main() -> sled::Result<()> {
    let db = sled::open("c69_example_sled_db")?; // creates the database directory on disk
    store(&db, "stock:paracetamol", "500 mg tablets, shelf A3")?;
    store(&db, "stock:insulin", "10 ml vials, fridge 2")?;
    db.flush()?; // persist to disk — the records survive a restart
    println!("[pharmacy] {} records stored", db.len());
    println!("[pharmacy] paracetamol on file? {}", db.contains_key("stock:paracetamol")?);

    drop(db); // close the database before removing its files
    std::fs::remove_dir_all("c69_example_sled_db").ok();
    Ok(())
}
