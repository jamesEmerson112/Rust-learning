use serde::{Deserialize, Serialize};

// Capstone: read a CSV (c47 skill), store each row as a struct in sled, then
// PROCESS the data — total, count, and the priciest service. anyhow unifies the
// csv + serde + sled error types so a single `?` style handles them all.
#[derive(Debug, Serialize, Deserialize)]
struct Service {
    name: String,
    price: u32,
}

fn import_and_total(db: &sled::Db, csv_path: &str) -> anyhow::Result<u32> {
    let mut rdr = csv::Reader::from_path(csv_path)?;
    for row in rdr.deserialize() {
        let svc: Service = row?;
        db.insert(svc.name.as_bytes(), serde_json::to_vec(&svc)?)?;
    }
    let mut total = 0;
    for item in db.iter() {
        let (_k, v) = item?;
        total += serde_json::from_slice::<Service>(&v)?.price;
    }
    Ok(total)
}

fn main() -> anyhow::Result<()> {
    std::fs::write(
        "c74_example.csv",
        "name,price\nGel Manicure,4500\nPedicure,3500\nAcrylic Full Set,6000\n",
    )?;
    let db = sled::open("c74_example_sled_db")?;
    let total = import_and_total(&db, "c74_example.csv")?;

    // process the stored data into a small report
    let mut services = Vec::new();
    for item in db.iter() {
        let (_k, v) = item?;
        services.push(serde_json::from_slice::<Service>(&v)?);
    }
    println!("imported {} services, total {total} cents", services.len());
    if let Some(top) = services.iter().max_by_key(|s| s.price) {
        println!("priciest: {} ({} cents)", top.name, top.price);
    }

    drop(db);
    std::fs::remove_dir_all("c74_example_sled_db").ok();
    std::fs::remove_file("c74_example.csv").ok();
    Ok(())
}
