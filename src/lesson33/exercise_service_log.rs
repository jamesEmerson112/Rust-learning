use std::collections::HashMap;

pub struct ServiceLog {
    revenue: HashMap<String, Vec<u32>>,
}

impl ServiceLog {
    pub fn new() -> Self {
        // TODO: Initialize an empty service log.
        Self {
            revenue: HashMap::new(),
        }
    }

    pub fn add_service(&mut self, technician: &str, price: u32) {
        // TODO: Push each price into the technician's revenue list.
        let _ = (technician, price);
    }

    pub fn average_revenue(&self, technician: &str) -> Option<f64> {
        // TODO: Return the technician's average revenue as f64, or None when missing.
        let _ = technician;
        let _ = &self.revenue;
        None
    }
}
