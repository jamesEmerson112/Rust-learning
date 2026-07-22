// Bug Hunt drill: iterator predicate + fold accumulator. filter() KEEPS the items whose
// predicate is true, and fold() folds them into an accumulator starting from a correct seed.
// Get the predicate direction right (== "VIP", not !=) and start the sum at 0.
// Coming from C: this is the `for (i=0; i<n; i++) if (tier[i]==VIP) sum += tip[i];` loop,
// but the `==` typo becomes a `!=` that quietly sums the WRONG half of the customers.
pub fn vip_tip_total(tips: &[(&str, u32)]) -> u32 {
    tips.iter()
        .filter(|(tier, _cents)| *tier == "VIP")
        .fold(0, |acc, (_tier, cents)| acc + cents)
}

fn main() {
    let jar = [
        ("VIP", 1000u32),
        ("Regular", 200),
        ("VIP", 1500),
        ("Regular", 300),
        ("VIP", 500),
    ];
    println!("VIP tips collected: {} cents", vip_tip_total(&jar));
}
