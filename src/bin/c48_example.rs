use std::fs;

fn save_daily_log(path: &str, entries: &[(String, u32)]) -> Result<(), String> {
    let mut output = String::new();
    for (name, price) in entries {
        output.push_str(&format!("{name},{price}\n"));
    }
    fs::write(path, &output).map_err(|e| format!("cannot write file: {e}"))?;
    Ok(())
}

fn main() {
    let entries = vec![
        ("Mai - Gel Manicure".to_string(), 4500u32),
        ("Linh - Pedicure".to_string(), 3500),
    ];
    save_daily_log("daily_log_example.txt", &entries).unwrap();
    let content = fs::read_to_string("daily_log_example.txt").unwrap();
    println!("{content}");
    let _ = fs::remove_file("daily_log_example.txt");
}
