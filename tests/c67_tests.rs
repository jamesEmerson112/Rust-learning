#[path = "../src/bin/c67_exercise.rs"]
#[allow(dead_code)]
mod c67_exercise;

use c67_exercise::pool_the_take;

#[test]
fn every_deposit_lands_in_the_shared_take() {
    // 10 runners × 100 creds — the ledger must read 1000, every run, no races
    assert_eq!(pool_the_take(), 1000);
}
