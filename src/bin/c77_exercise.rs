// BUG: The tip jar is supposed to total the VIP clients' tips, but the fancy tips keep
// vanishing — the number that comes out is the walk-in regulars' tips instead. Somewhere the
// filter is keeping exactly the wrong customers. The code compiles and runs. Find and fix it.
// (This drills c21-c24: filter + fold. The tests in tests/c77_tests.rs must go green.)
pub fn vip_tip_total(tips: &[(&str, u32)]) -> u32 {
    tips.iter()
        .filter(|(tier, _cents)| *tier != "VIP")
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
