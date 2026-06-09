use serde::{Deserialize, Serialize};

// "Querying" without SQL: load the records, then process them with the iterator
// combinators from c21-22/c45-46 — filter() selects rows, map() projects a field.
#[derive(Debug, Serialize, Deserialize)]
struct Service {
    name: String,
    price: u32,
}

fn load_all(db: &sled::Db) -> anyhow::Result<Vec<Service>> {
    let mut out = Vec::new();
    for item in db.iter() {
        let (_k, v) = item?;
        out.push(serde_json::from_slice::<Service>(&v)?);
    }
    Ok(out)
}

fn services_over(db: &sled::Db, min_price: u32) -> anyhow::Result<Vec<Service>> {
    let mut out: Vec<Service> = load_all(db)?
        .into_iter()
        .filter(|s| s.price >= min_price)
        .collect();
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

fn names_over(db: &sled::Db, min_price: u32) -> anyhow::Result<Vec<String>> {
    let mut names: Vec<String> = load_all(db)?
        .into_iter()
        .filter(|s| s.price >= min_price)
        .map(|s| s.name)
        .collect();
    names.sort();
    Ok(names)
}

fn main() -> anyhow::Result<()> {
    let db = sled::open("c73_example_sled_db")?;
    for (name, price) in [("Gel Manicure", 4500u32), ("Pedicure", 3500), ("Acrylic", 6000)] {
        db.insert(name, serde_json::to_vec(&Service { name: name.to_string(), price })?)?;
    }
    println!("names over 4000: {:?}", names_over(&db, 4000)?);
    for svc in services_over(&db, 4000)? {
        println!("{} - {} cents", svc.name, svc.price);
    }
    drop(db);
    std::fs::remove_dir_all("c73_example_sled_db").ok();
    Ok(())
}
