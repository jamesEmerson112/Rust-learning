// Bug Hunt drill: iterator predicate + fold accumulator. filter() KEEPS the items whose
// predicate is true, and fold() folds them into an accumulator starting from a correct seed.
// Get the predicate direction right (== "Critical", not !=) and start the sum at 0.
// Coming from C: this is the `for (i=0; i<n; i++) if (status[i]==CRITICAL) sum += flow[i];`
// loop, but the `==` typo becomes a `!=` that quietly sums the WRONG half of the ward.
pub fn critical_oxygen_total(beds: &[(&str, u32)]) -> u32 {
    beds.iter()
        .filter(|(status, _lpm)| *status == "Critical")
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
