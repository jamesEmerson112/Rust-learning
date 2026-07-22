// BUG: A typo slipped into the printed price list — "Pedicure,oops" instead of a number.
// Instead of refusing the corrupt list, daily_total quietly treats the bad row as 0 cents
// and hands back a total that's too low. The books balance to a lie. The code compiles and
// runs — a corrupt price must come back as an Err, not vanish. Find and fix it.
// (This drills c17-c19: Result / map_err / ?. The tests in tests/c76_tests.rs must go green.)
pub fn daily_total(rows: &[&str]) -> Result<u32, String> {
    let mut total = 0;
    for row in rows {
        let price_str = row
            .split(',')
            .nth(1)
            .ok_or_else(|| format!("malformed row: {row}"))?;
        let price = price_str.trim().parse::<u32>().unwrap_or(0);
        total += price;
    }
    Ok(total)
}

fn main() {
    let corrupt = ["Gel Manicure,4500", "Pedicure,oops", "Acrylic Full Set,6000"];
    println!("today's total: {:?}", daily_total(&corrupt));
}
