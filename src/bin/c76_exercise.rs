// BUG: A smudged entry made it onto the fluid chart. It reads "Water,oops" instead of a
// number. Instead of rejecting the chart, fluid_intake quietly counts the bad row as 0 ml and
// reports an intake that is too low. The doctor would think the patient drank less than they
// really did. The code compiles and runs. The tests expect a corrupt amount to produce an
// Err. Find the bug and fix it.
// This drills c17 to c19. The tests in tests/c76_tests.rs must pass.
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
