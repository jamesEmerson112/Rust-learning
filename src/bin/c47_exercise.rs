use std::fs;

pub fn load_price_list(path: &str) -> Result<Vec<(String, u32)>, String> {
    // TODO: Read the file at `path` with fs::read_to_string, map_err to String.
    // Each line is "ServiceName,Price". Split on ',', parse the price.
    // Return a Vec of (name, price) tuples.
    let _ = path;
    Ok(Vec::new())
}

fn main() {
    let _ = fs::write("services_test.txt", "Gel Manicure,4500\n");
    println!("{:?}", load_price_list("services_test.txt"));
    let _ = fs::remove_file("services_test.txt");
}
