#[allow(unused_imports)]
use csv::Reader;
use serde::Deserialize;
use std::{fs, net::Shutdown::Read};

#[derive(Debug, PartialEq, Deserialize)]
pub struct Service {
    pub name: String,
    pub price: u32,
}

pub fn load_price_list(path: &str) -> Result<Vec<Service>, String> {
    // TODO: Open the CSV at `path` with csv::Reader::from_path (map_err to String).
    // Loop over rdr.deserialize(); each row deserializes into a Service — the
    // header row maps columns -> fields, so no splitn/parse needed. Collect a Vec.
    let mut rdr = Reader::from_path(path).map_err(|e| format!("cannot read files: {e}"))?;
    let mut items = Vec::new();
    for row in rdr.deserialize() {
        let svc: Service = row.map_err(|e| format!("bad row: {e}"))?;
        items.push(svc);
    }

    Ok(items)
}

fn main() {
    let _ = fs::write("services_test.csv", "name,price\nGel Manicure,4500\n");
    println!("{:?}", load_price_list("services_test.csv"));
    let _ = fs::remove_file("services_test.csv");
}
