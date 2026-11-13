//! Tests for the toilet stall in `src/bin/d02_toilet.rs`.
//!
//! Contents:
//! - lock tests: taking and leaving the stall
//! - flush tests: water use and a low tank
//! - refill test: the tank fills each tick up to capacity
//! - out-of-order test: a clog closes the stall until it is repaired

#[path = "../src/bin/d02_toilet.rs"]
#[allow(dead_code)]
mod d02_toilet;

use d02_toilet::{Flush, Stall, Toilet, ToiletError};

/// Description: A new toilet is vacant, has a full tank and has used no water.
/// Inputs: none
/// Outputs: none
#[test]
fn starts_vacant_with_a_full_tank() {
    let toilet = Toilet::new(9, 2);
    assert_eq!(toilet.stall(), Stall::Vacant);
    assert_eq!(toilet.tank(), 9);
    assert_eq!(toilet.liters_used(), 0);
}

/// Description: Locking occupies the stall, and unlocking frees it again.
/// Inputs: none
/// Outputs: none
#[test]
fn lock_then_unlock() {
    let mut toilet = Toilet::new(9, 2);
    assert_eq!(toilet.lock(), Ok(()));
    assert_eq!(toilet.stall(), Stall::Occupied);
    assert_eq!(toilet.unlock(), Ok(()));
    assert_eq!(toilet.stall(), Stall::Vacant);
}

/// Description: A second visitor cannot lock a stall that is already occupied.
/// Inputs: none
/// Outputs: none
#[test]
fn an_occupied_stall_cannot_be_locked_again() {
    let mut toilet = Toilet::new(9, 2);
    toilet.lock().unwrap();
    assert_eq!(toilet.lock(), Err(ToiletError::AlreadyOccupied));
}

/// Description: A vacant stall has nobody inside to unlock it.
/// Inputs: none
/// Outputs: none
#[test]
fn a_vacant_stall_cannot_be_unlocked() {
    let mut toilet = Toilet::new(9, 2);
    assert_eq!(toilet.unlock(), Err(ToiletError::NotOccupied));
}

/// Description: A half flush takes 3 liters, a full flush takes 6, and the meter counts both.
/// Inputs: none
/// Outputs: none
#[test]
fn half_and_full_flushes_use_3_and_6_liters() {
    let mut toilet = Toilet::new(9, 2);
    assert_eq!(toilet.flush(Flush::Half), Ok(6));
    assert_eq!(toilet.flush(Flush::Full), Ok(0));
    assert_eq!(toilet.liters_used(), 9);
}

/// Description: A flush the tank cannot cover is refused, and the tank and meter stay unchanged.
/// Inputs: none
/// Outputs: none
#[test]
fn a_low_tank_refuses_the_flush() {
    let mut toilet = Toilet::new(6, 2);
    toilet.flush(Flush::Full).unwrap();
    assert_eq!(toilet.flush(Flush::Half), Err(ToiletError::TankLow { needed: 3, available: 0 }));
    assert_eq!(toilet.tank(), 0);
    assert_eq!(toilet.liters_used(), 6);
}

/// Description: Each tick adds the refill amount until the tank reaches capacity.
/// Inputs: none
/// Outputs: none
#[test]
fn the_tank_refills_each_tick_up_to_capacity() {
    let mut toilet = Toilet::new(6, 4);
    toilet.flush(Flush::Full).unwrap();
    toilet.tick();
    assert_eq!(toilet.tank(), 4);
    toilet.tick();
    assert_eq!(toilet.tank(), 6);
    toilet.tick();
    assert_eq!(toilet.tank(), 6);
}

/// Description: A clog refuses locking, unlocking and flushing until a repair makes the stall
/// vacant again.
/// Inputs: none
/// Outputs: none
#[test]
fn out_of_order_refuses_everything_until_repaired() {
    let mut toilet = Toilet::new(9, 2);
    toilet.lock().unwrap();
    toilet.report_clog();
    assert_eq!(toilet.stall(), Stall::OutOfOrder);
    assert_eq!(toilet.lock(), Err(ToiletError::OutOfOrder));
    assert_eq!(toilet.unlock(), Err(ToiletError::OutOfOrder));
    assert_eq!(toilet.flush(Flush::Half), Err(ToiletError::OutOfOrder));
    toilet.repair();
    assert_eq!(toilet.stall(), Stall::Vacant);
    assert_eq!(toilet.lock(), Ok(()));
}
