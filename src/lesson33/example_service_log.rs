use std::collections::HashMap;

pub struct ServiceLog {
    revenue: HashMap<String, Vec<u32>>,
}

impl ServiceLog {
    pub fn new() -> Self {
        Self {
            revenue: HashMap::new(),
        }
    }

    pub fn add_service(&mut self, technician: &str, price: u32) {
        self.revenue
            .entry(technician.to_string())
            .or_default()
            .push(price);
    }

    pub fn average_revenue(&self, technician: &str) -> Option<f64> {
        let prices = self.revenue.get(technician)?;
        if prices.is_empty() {
            return None;
        }

        let sum: u32 = prices.iter().sum();
        Some(sum as f64 / prices.len() as f64)
    }
}
