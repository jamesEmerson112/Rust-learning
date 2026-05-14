fn daily_revenue(prices: &[u32]) -> u32 {
    prices.iter().sum()
}

fn main() {
    let services = [4500, 3500, 6000, 2500];
    println!("Today's revenue: {} cents", daily_revenue(&services));
}
