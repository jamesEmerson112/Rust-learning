// Bug Hunt drill: DON'T swallow parse errors. A bad row must surface as Err, not silently
// become 0. `?` on map_err propagates the failure so a corrupt fluid chart refuses to lie
// about the day's intake instead of quietly undercounting it.
// Coming from C: unwrap_or(0) is `atoi(s)` — it returns 0 for garbage and you never know.
// parse()?.map_err is `strtol` with the errno actually checked and the caller told.
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
