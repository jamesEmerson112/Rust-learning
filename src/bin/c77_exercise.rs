// BUG: The oxygen report is supposed to total the flow going to the CRITICAL beds, but the
// number that comes out is the stable patients' total instead. The sickest patients vanish
// from the report. Somewhere the filter is keeping exactly the wrong beds. The code compiles
// and runs. Find and fix it.
// (This drills c21-c24: filter + fold. The tests in tests/c77_tests.rs must go green.)
pub fn critical_oxygen_total(beds: &[(&str, u32)]) -> u32 {
    beds.iter()
        .filter(|(status, _lpm)| *status != "Critical")
        .fold(0, |acc, (_status, lpm)| acc + lpm)
}

fn main() {
    let ward = [
        ("Critical", 10u32),
        ("Stable", 2),
        ("Critical", 15),
        ("Stable", 3),
        ("Critical", 5),
    ];
    println!("oxygen to critical beds: {} L/min", critical_oxygen_total(&ward));
}
