#[path = "../src/bin/c77_exercise.rs"]
#[allow(dead_code)]
mod c77_exercise;

use c77_exercise::*;

#[test]
fn sums_only_vip_tips() {
    let jar = [
        ("VIP", 1000u32),
        ("Regular", 200),
        ("VIP", 1500),
        ("Regular", 300),
        ("VIP", 500),
    ];
    assert_eq!(vip_tip_total(&jar), 3000);
}

#[test]
fn no_vip_tips_is_zero() {
    let jar = [("Regular", 200u32), ("Regular", 300)];
    assert_eq!(vip_tip_total(&jar), 0);
}

#[test]
fn empty_jar_is_zero() {
    let jar: [(&str, u32); 0] = [];
    assert_eq!(vip_tip_total(&jar), 0);
}
