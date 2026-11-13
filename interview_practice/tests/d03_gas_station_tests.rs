//! Tests for the gas station in `src/bin/d03_gas_station.rs`.
//!
//! Contents:
//! - `regular_station`: a helper that builds a two-pump station selling regular fuel
//! - sale tests: charging, refunds and whole liters
//! - limit tests: the prepayment and the tank both stop the pump
//! - pump tests: busy, unknown and unauthorized pumps
//! - price tests: unpriced grades and prices fixed at authorization

#[path = "../src/bin/d03_gas_station.rs"]
#[allow(dead_code)]
mod d03_gas_station;

use d03_gas_station::{Grade, Receipt, Station, StationError};

/// Description: Builds a two-pump station that sells regular fuel.
/// Inputs:
/// - `cents_per_liter`: the price of regular.
/// - `liters`: the liters in the regular tank.
/// Outputs: the station.
fn regular_station(cents_per_liter: u32, liters: u32) -> Station {
    let mut station = Station::new(2);
    station.set_price(Grade::Regular, cents_per_liter);
    station.deliver(Grade::Regular, liters);
    station
}

/// Description: The customer pays for the 10 liters pumped and gets the rest of the prepayment
/// back.
/// Inputs: none
/// Outputs: none
#[test]
fn a_sale_charges_only_for_the_fuel_pumped() {
    let mut station = regular_station(200, 1_000);
    station.authorize(0, Grade::Regular, 5_000).unwrap();
    assert_eq!(station.pump(0, 10), Ok(10));
    assert_eq!(
        station.finish(0),
        Ok(Receipt { grade: Grade::Regular, liters: 10, charged_cents: 2_000, refund_cents: 3_000 })
    );
    assert_eq!(station.revenue_cents(), 2_000);
    assert_eq!(station.tank_level(Grade::Regular), 990);
}

/// Description: A 50 dollar prepayment at 2 dollars a liter stops the pump at 25 liters.
/// Inputs: none
/// Outputs: none
#[test]
fn pumping_stops_when_the_prepayment_runs_out() {
    let mut station = regular_station(200, 1_000);
    station.authorize(0, Grade::Regular, 5_000).unwrap();
    assert_eq!(station.pump(0, 40), Ok(25));
    assert_eq!(station.pump(0, 5), Ok(0));
    let receipt = station.finish(0).unwrap();
    assert_eq!(receipt.charged_cents, 5_000);
    assert_eq!(receipt.refund_cents, 0);
}

/// Description: A prepayment buys whole liters only, and the leftover cents are refunded.
/// Inputs: none
/// Outputs: none
#[test]
fn the_prepayment_buys_whole_liters_only() {
    let mut station = regular_station(180, 1_000);
    station.authorize(0, Grade::Regular, 4_000).unwrap();
    assert_eq!(station.pump(0, 30), Ok(22));
    let receipt = station.finish(0).unwrap();
    assert_eq!(receipt.charged_cents, 3_960);
    assert_eq!(receipt.refund_cents, 40);
}

/// Description: The pump stops when the underground tank runs dry.
/// Inputs: none
/// Outputs: none
#[test]
fn pumping_stops_when_the_tank_runs_dry() {
    let mut station = regular_station(200, 8);
    station.authorize(0, Grade::Regular, 10_000).unwrap();
    assert_eq!(station.pump(0, 20), Ok(8));
    assert_eq!(station.pump(0, 5), Ok(0));
    assert_eq!(station.tank_level(Grade::Regular), 0);
}

/// Description: A pump that is serving someone refuses a second customer, while the other pump
/// stays free.
/// Inputs: none
/// Outputs: none
#[test]
fn a_busy_pump_refuses_a_second_customer() {
    let mut station = regular_station(200, 1_000);
    station.authorize(0, Grade::Regular, 1_000).unwrap();
    assert_eq!(station.authorize(0, Grade::Regular, 1_000), Err(StationError::PumpBusy(0)));
    assert_eq!(station.authorize(1, Grade::Regular, 1_000), Ok(()));
}

/// Description: A pump number that does not exist, or a pump nobody has paid at, is refused.
/// Inputs: none
/// Outputs: none
#[test]
fn unknown_and_unauthorized_pumps_are_refused() {
    let mut station = regular_station(200, 1_000);
    assert_eq!(station.authorize(7, Grade::Regular, 1_000), Err(StationError::NoSuchPump(7)));
    assert_eq!(station.pump(7, 5), Err(StationError::NoSuchPump(7)));
    assert_eq!(station.pump(1, 5), Err(StationError::NotAuthorized(1)));
    assert_eq!(station.finish(1), Err(StationError::NotAuthorized(1)));
}

/// Description: Diesel has no price, so no pump can sell it.
/// Inputs: none
/// Outputs: none
#[test]
fn a_grade_without_a_price_cannot_be_sold() {
    let mut station = regular_station(200, 1_000);
    assert_eq!(
        station.authorize(0, Grade::Diesel, 1_000),
        Err(StationError::NoPrice(Grade::Diesel))
    );
}

/// Description: A price change after authorization does not change what that sale costs.
/// Inputs: none
/// Outputs: none
#[test]
fn the_price_is_fixed_when_the_pump_is_authorized() {
    let mut station = regular_station(200, 1_000);
    station.authorize(0, Grade::Regular, 5_000).unwrap();
    station.set_price(Grade::Regular, 300);
    station.pump(0, 10).unwrap();
    assert_eq!(station.finish(0).unwrap().charged_cents, 2_000);
}

/// Description: Finishing a sale frees the pump for the next customer.
/// Inputs: none
/// Outputs: none
#[test]
fn a_finished_pump_serves_the_next_customer() {
    let mut station = regular_station(200, 1_000);
    station.authorize(0, Grade::Regular, 1_000).unwrap();
    station.finish(0).unwrap();
    assert_eq!(station.authorize(0, Grade::Regular, 1_000), Ok(()));
}
