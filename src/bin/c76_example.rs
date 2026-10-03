// Bug Hunt drill: do not hide parse errors. A bad row should come back as an Err instead of
// silently turning into 0. map_err converts the parse error into the function's error type,
// and `?` passes it back to the caller. A corrupt fluid chart then reports an error instead
// of quietly undercounting the day's intake.
// Coming from C: unwrap_or(0) is like `atoi(s)`, which returns 0 for invalid input without
// telling you. parse() with map_err and `?` is like `strtol` where you check errno and report
// the failure to the caller.
pub fn fluid_intake(rows: &[&str]) -> Result<u32, String> {
    let mut total = 0;
    for row in rows {
        let ml_str = row
            .split(',')
            .nth(1)
            .ok_or_else(|| format!("malformed row: {row}"))?;
        let ml = ml_str
            .trim()
            .parse::<u32>()
            .map_err(|_| format!("bad amount: {ml_str}"))?;
        total += ml;
    }
    Ok(total)
}

fn main() {
    let clean = ["IV saline,500", "Water,250", "Soup,300"];
    println!("clean chart intake: {:?}", fluid_intake(&clean));

    let corrupt = ["IV saline,500", "Water,oops", "Soup,300"];
    println!("corrupt chart intake: {:?}", fluid_intake(&corrupt));
}
