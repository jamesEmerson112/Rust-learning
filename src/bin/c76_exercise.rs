// BUG: A smudged entry made it onto the fluid chart — "Water,oops" instead of a number.
// Instead of rejecting the chart, fluid_intake quietly counts the bad row as 0 ml and reports
// an intake that's too low. The doctor would see a patient drinking less than they really did.
// The code compiles and runs — a corrupt amount must come back as an Err, not vanish. Find and fix it.
// (This drills c17-c19: Result / map_err / ?. The tests in tests/c76_tests.rs must go green.)
pub fn fluid_intake(rows: &[&str]) -> Result<u32, String> {
    let mut total = 0;
    for row in rows {
        let ml_str = row
            .split(',')
            .nth(1)
            .ok_or_else(|| format!("malformed row: {row}"))?;
        let ml = ml_str.trim().parse::<u32>().unwrap_or(0);
        total += ml;
    }
    Ok(total)
}

fn main() {
    let corrupt = ["IV saline,500", "Water,oops", "Soup,300"];
    println!("today's intake: {:?}", fluid_intake(&corrupt));
}
