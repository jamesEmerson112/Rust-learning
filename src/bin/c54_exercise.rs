use std::collections::HashMap;

pub struct Ward {
    visits: Vec<(String, String, u32)>,
}

impl Ward {
    pub fn new() -> Self {
        Self { visits: Vec::new() }
    }

    pub fn log_visit(&mut self, nurse: &str, patient: &str, minutes: u32) {
        // TODO: Push a tuple (nurse, patient, minutes) into visits.
        let _ = (nurse, patient, minutes);
        self.visits.push((nurse.to_string(), patient.to_string(), minutes));
    }

    pub fn list(&self) -> &[(String, String, u32)] {
        // TODO: Return a slice of all visits.
        &self.visits
    }

    pub fn minutes_by_nurse(&self) -> HashMap<String, u32> {
        // TODO: Sum care minutes per nurse and return a HashMap.
        let mut list = HashMap::new();
        for (nurse, _, minutes) in &self.visits {
            *list.entry(nurse.clone()).or_insert(0) += minutes;
        }
        list
    }
}

fn main() {
    let mut ward = Ward::new();
    ward.log_visit("Mai", "Mr. Hung", 45);
    println!("Visits logged: {}", ward.list().len());
}
