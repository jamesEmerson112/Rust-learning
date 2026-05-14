pub fn daily_revenue(prices: &[u32]) -> u32 {
    // TODO: Sum all the prices in the slice and return the total.
    let _ = prices;
    0
}

fn main() {
    let services = [4500, 3500, 6000];
    println!("Revenue: {} cents", daily_revenue(&services));
}
