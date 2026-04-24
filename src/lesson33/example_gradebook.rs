use std::collections::HashMap;

pub struct Gradebook {
    scores: HashMap<String, Vec<u32>>,
}

impl Gradebook {
    pub fn new() -> Self {
        Self {
            scores: HashMap::new(),
        }
    }

    pub fn add_score(&mut self, student: &str, score: u32) {
        self.scores
            .entry(student.to_string())
            .or_default()
            .push(score);
    }

    pub fn average(&self, student: &str) -> Option<f64> {
        let scores = self.scores.get(student)?;
        if scores.is_empty() {
            return None;
        }

        let sum: u32 = scores.iter().sum();
        Some(sum as f64 / scores.len() as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::Gradebook;

    #[test]
    fn average_is_computed() {
        let mut gradebook = Gradebook::new();
        gradebook.add_score("Ava", 80);
        gradebook.add_score("Ava", 100);

        let avg = gradebook.average("Ava").expect("expected average");
        assert!((avg - 90.0).abs() < f64::EPSILON);
    }

    #[test]
    fn unknown_student_is_none() {
        let gradebook = Gradebook::new();
        assert_eq!(gradebook.average("Missing"), None);
    }
}
