use serde::{Deserialize, Serialize};

// Scan the whole store. db.iter() yields Result<(IVec, IVec)> pairs (key, value);
// take the value, decode it, collect. Sort for a deterministic order.
#[derive(Debug, Serialize, Deserialize)]
struct Service {
    name: String,
    price: u32,
}

fn all_services(db: &sled::Db) -> anyhow::Result<Vec<Service>> {
    let mut out = Vec::new();
    for item in db.iter() {
        let (_key, value) = item?;
        out.push(serde_json::from_slice::<Service>(&value)?);
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

fn main() -> anyhow::Result<()> {
    let db = sled::open("c72_example_sled_db")?;
    for (name, price) in [("Gel Manicure", 4500u32), ("Pedicure", 3500), ("Acrylic", 6000)] {
        db.insert(name, serde_json::to_vec(&Service { name: name.to_string(), price })?)?;
    }
    for svc in all_services(&db)? {
        println!("{} - {} cents", svc.name, svc.price);
    }
    drop(db);
    std::fs::remove_dir_all("c72_example_sled_db").ok();
    Ok(())
}
