// Bug Hunt drill: an iterator filter followed by fold. filter() keeps the items for which the
// test function, called the predicate, returns true. fold() then combines those items into
// one running value, starting from an initial value you choose. Here the predicate must be
// == "Critical", not !=, and the sum must start at 0.
// Coming from C: this is the same as
// `for (i=0; i<n; i++) if (status[i]==CRITICAL) sum += flow[i];`. A typo that turns `==`
// into `!=` makes the loop quietly sum the wrong half of the ward.
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
