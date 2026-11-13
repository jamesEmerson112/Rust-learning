//! Tests for the nail salon pay system in `src/bin/d04_nail_salon_pay.rs`.
//!
//! Contents:
//! - `open_salon`: a helper that builds a salon with two services and two technicians
//! - split tests: commission, tips, card fees and rounding
//! - refusal test: bad tickets change no balance
//! - payroll tests: earnings across tickets and the end-of-day payout

#[path = "../src/bin/d04_nail_salon_pay.rs"]
#[allow(dead_code)]
mod d04_nail_salon_pay;

use d04_nail_salon_pay::{PayError, Payment, Receipt, Salon};

/// Description: Builds a salon with two services and two technicians.
/// Inputs: none
/// Outputs: a salon where a gel manicure costs 35 dollars and a pedicure 45, Mai earns 60 percent
/// commission and Linh earns 50 percent.
fn open_salon() -> Salon {
    let mut salon = Salon::new();
    salon.add_service("gel manicure", 3_500);
    salon.add_service("pedicure", 4_500);
    salon.hire("Mai", 60);
    salon.hire("Linh", 50);
    salon
}

/// Description: A cash ticket gives the technician 60 percent of the services plus the whole tip,
/// and the salon keeps the rest.
/// Inputs: none
/// Outputs: none
#[test]
fn a_cash_ticket_pays_commission_plus_the_whole_tip() {
    let mut salon = open_salon();
    let receipt = salon.checkout("Mai", &["gel manicure", "pedicure"], 1_000, Payment::Cash);
    assert_eq!(
        receipt,
        Ok(Receipt {
            subtotal_cents: 8_000,
            tip_cents: 1_000,
            charged_cents: 9_000,
            card_fee_cents: 0,
            technician_cents: 5_800,
            salon_cents: 3_200,
        })
    );
}

/// Description: The 3 percent card fee on the whole charge comes out of the salon's share, not
/// the technician's.
/// Inputs: none
/// Outputs: none
#[test]
fn the_card_fee_comes_out_of_the_salon_share() {
    let mut salon = open_salon();
    let receipt =
        salon.checkout("Mai", &["gel manicure", "pedicure"], 1_000, Payment::Card).unwrap();
    assert_eq!(receipt.card_fee_cents, 270);
    assert_eq!(receipt.technician_cents, 5_800);
    assert_eq!(receipt.salon_cents, 2_930);
}

/// Description: A commission and a card fee that land on fractions of a cent round down.
/// Inputs: none
/// Outputs: none
#[test]
fn commission_and_fees_round_down_to_whole_cents() {
    let mut salon = open_salon();
    salon.add_service("polish change", 1_999);
    salon.hire("Thu", 55);
    let receipt = salon.checkout("Thu", &["polish change"], 0, Payment::Card).unwrap();
    assert_eq!(receipt.technician_cents, 1_099);
    assert_eq!(receipt.card_fee_cents, 59);
    assert_eq!(receipt.salon_cents, 841);
}

/// Description: An empty ticket, an unknown technician and an unknown service are refused, and
/// none of them changes a balance.
/// Inputs: none
/// Outputs: none
#[test]
fn refused_tickets_change_no_balance() {
    let mut salon = open_salon();
    assert_eq!(salon.checkout("Mai", &[], 500, Payment::Cash), Err(PayError::EmptyTicket));
    assert_eq!(
        salon.checkout("Hoa", &["pedicure"], 0, Payment::Cash),
        Err(PayError::UnknownTechnician("Hoa".to_string()))
    );
    assert_eq!(
        salon.checkout("Mai", &["gel manicure", "acrylic set"], 0, Payment::Cash),
        Err(PayError::UnknownService("acrylic set".to_string()))
    );
    assert_eq!(salon.earnings("Mai"), 0);
    assert_eq!(salon.salon_net_cents(), 0);
}

/// Description: Two tickets for Mai add up in Mai's earnings and in the salon's net.
/// Inputs: none
/// Outputs: none
#[test]
fn earnings_add_up_across_tickets() {
    let mut salon = open_salon();
    salon.checkout("Mai", &["gel manicure"], 500, Payment::Cash).unwrap();
    salon.checkout("Mai", &["pedicure"], 0, Payment::Cash).unwrap();
    assert_eq!(salon.earnings("Mai"), 5_300);
    assert_eq!(salon.salon_net_cents(), 3_200);
}

/// Description: The end-of-day payout lists each technician by name and then zeroes every
/// balance.
/// Inputs: none
/// Outputs: none
#[test]
fn end_of_day_pays_each_technician_by_name_and_resets() {
    let mut salon = open_salon();
    salon.checkout("Mai", &["gel manicure"], 500, Payment::Cash).unwrap();
    salon.checkout("Linh", &["pedicure"], 0, Payment::Cash).unwrap();
    assert_eq!(
        salon.end_of_day(),
        vec![("Linh".to_string(), 2_250), ("Mai".to_string(), 2_600)]
    );
    assert_eq!(salon.earnings("Mai"), 0);
    assert_eq!(salon.salon_net_cents(), 0);
    assert!(salon.end_of_day().is_empty());
}
