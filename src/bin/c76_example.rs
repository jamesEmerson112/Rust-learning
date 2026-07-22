// Bug Hunt drill: DON'T swallow parse errors. A bad row must surface as Err, not silently
// become 0. `?` on map_err propagates the failure so a corrupt price list refuses to lie
// about the day's total instead of quietly undercounting it.
// Coming from C: unwrap_or(0) is `atoi(s)` — it returns 0 for garbage and you never know.
// parse()?.map_err is `strtol` with the errno actually checked and the caller told.
pub fn daily_total(rows: &[&str]) -> Result<u32, String> {
    let mut total = 0;
    for row in rows {
        let price_str = row
            .split(',')
            .nth(1)
            .ok_or_else(|| format!("malformed row: {row}"))?;
        let price = price_str
            .trim()
            .parse::<u32>()
            .map_err(|_| format!("bad price: {price_str}"))?;
        total += price;
    }
    Ok(total)
}

fn main() {
    let clean = ["Gel Manicure,4500", "Pedicure,3500", "Acrylic Full Set,6000"];
    println!("clean list total: {:?}", daily_total(&clean));

    let corrupt = ["Gel Manicure,4500", "Pedicure,oops", "Acrylic Full Set,6000"];
    println!("corrupt list total: {:?}", daily_total(&corrupt));
}
