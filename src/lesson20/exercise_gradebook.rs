use std::collections::HashMap;

pub struct Gradebook {
    scores: HashMap<String, Vec<u32>>,
}

impl Gradebook {
    pub fn new() -> Self {
        // TODO: Initialize an empty gradebook.
        Self {
            scores: HashMap::new(),
        }
    }

    pub fn add_score(&mut self, student: &str, score: u32) {
        // TODO: Push each score into the student's score list.
        let _ = (student, score);
    }

    pub fn average(&self, student: &str) -> Option<f64> {
        // TODO: Return the student's average as f64, or None when missing.
        let _ = student;
        let _ = &self.scores;
        None
    }
}
