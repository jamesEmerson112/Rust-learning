use std::fs;

pub fn save_daily_log(path: &str, entries: &[(String, u32)]) -> Result<(), String> {
    // TODO: Build a string with one "name,price\n" per entry.
    // Write it to the file at `path` using fs::write, map_err to String.
    let _ = (path, entries);
    Ok(())
}

fn main() {
    let entries = vec![("Mai - Gel".to_string(), 4500u32)];
    save_daily_log("daily_log_test.txt", &entries).unwrap();
    let _ = fs::remove_file("daily_log_test.txt");
}
