// Read values back out of sled by key. get() returns Option<IVec> (a smart
// pointer to bytes) — None when the key is absent, just like HashMap::get.
//
// RUST GENERAL HOSPITAL: a nurse looks a drug up before giving it. A missing key means
// the pharmacy doesn't stock that drug — an answer, not a crash.
fn fetch(db: &sled::Db, key: &str) -> sled::Result<Option<String>> {
    Ok(db.get(key)?.map(|v| String::from_utf8_lossy(&v).to_string()))
}

fn main() -> sled::Result<()> {
    let db = sled::open("c70_example_sled_db")?;
    db.insert("stock:paracetamol", "500 mg tablets, shelf A3".as_bytes())?;
    println!("[pharmacy] paracetamol -> {:?}", fetch(&db, "stock:paracetamol")?);
    println!("[pharmacy] unknown     -> {:?}", fetch(&db, "stock:unknown")?);
    drop(db);
    std::fs::remove_dir_all("c70_example_sled_db").ok();
    Ok(())
}
