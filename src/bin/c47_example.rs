use csv::Reader;
use serde::Deserialize;
use std::fs;

// The csv crate maps each column to a struct field by name (matching the
// header row). No splitn, no parts[0]/parts[1], no manual parse.
#[derive(Debug, Deserialize)]
struct Service {
    name: String,
    price: u32,
}

fn load_price_list(path: &str) -> Result<Vec<Service>, String> {
    let mut rdr = Reader::from_path(path).map_err(|e| format!("cannot read file: {e}"))?;
    let mut items = Vec::new();
    for row in rdr.deserialize() {
        let svc: Service = row.map_err(|e| format!("bad row: {e}"))?;
        items.push(svc);
    }
    Ok(items)
}

fn main() {
    let _ = fs::write(
        "services_example.csv",
        "name,price\nGel Manicure,4500\nPedicure,3500\nAcrylic Full Set,6000\n",
    );
    match load_price_list("services_example.csv") {
        Ok(list) => {
            for svc in &list {
                println!("{}: {} cents", svc.name, svc.price);
            }
        }
        Err(e) => println!("Error: {e}"),
    }
    let _ = fs::remove_file("services_example.csv");
}
