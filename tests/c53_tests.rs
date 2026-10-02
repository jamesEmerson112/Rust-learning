#[path = "../src/bin/c53_exercise.rs"]
#[allow(dead_code)]
mod c53_exercise;

use c53_exercise::{format_order, Args};

#[test]
fn formats_an_order() {
    let args = Args {
        patient: "Mr. Hung".to_string(),
        medication: "Paracetamol".to_string(),
        dose_mg: 500,
    };
    assert_eq!(format_order(&args), "Order: Paracetamol 500 mg for Mr. Hung");
}

#[test]
fn formats_another() {
    let args = Args {
        patient: "Mrs. Lan".to_string(),
        medication: "Amoxicillin".to_string(),
        dose_mg: 250,
    };
    assert_eq!(format_order(&args), "Order: Amoxicillin 250 mg for Mrs. Lan");
}
