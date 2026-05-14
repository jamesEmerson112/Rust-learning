use std::fs;

fn load_price_list(path: &str) -> Result<Vec<(String, u32)>, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("cannot read file: {e}"))?;
    let mut items = Vec::new();
    for line in content.lines() {
        let parts: Vec<&str> = line.splitn(2, ',').collect();
        if parts.len() == 2 {
            let name = parts[0].trim().to_string();
            let price: u32 = parts[1].trim().parse().map_err(|e| format!("bad price: {e}"))?;
            items.push((name, price));
        }
    }
    Ok(items)
}

fn main() {
    let _ = fs::write("services_example.txt", "Gel Manicure,4500\nPedicure,3500\nAcrylic Full Set,6000\n");
    match load_price_list("services_example.txt") {
        Ok(list) => {
            for (name, price) in &list {
                println!("{name}: {price} cents");
            }
        }
        Err(e) => println!("Error: {e}"),
    }
    let _ = fs::remove_file("services_example.txt");
}
