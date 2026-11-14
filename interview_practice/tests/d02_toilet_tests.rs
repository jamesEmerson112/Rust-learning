//! Tests for the smart bidet in `src/bin/d02_toilet.rs`.
//!
//! Contents:
//! - seat tests: sitting twice, an empty seat, standing up and the stop button
//! - wash tests: water by pressure and time, the pressure range, a low tank and a busy bidet
//! - tick tests: the countdown back to idle and the capped refill

#[path = "../src/bin/d02_toilet.rs"]
#[allow(dead_code)]
mod d02_toilet;

use d02_toilet::{Bidet, BidetError, MAX_PRESSURE, MIN_PRESSURE, Mode, Spray};

/// Description: Sitting down a second time without standing up is refused.
/// Inputs: none
/// Outputs: none
#[test]
fn sitting_twice_is_refused() {
    let mut bidet = Bidet::new(1_000, 50);
    assert_eq!(bidet.sit(), Ok(()));
    assert_eq!(bidet.sit(), Err(BidetError::AlreadySeated));
}

/// Description: With nobody seated, the bidet refuses to wash, dry or register a stand, and it
/// sprays no water.
/// Inputs: none
/// Outputs: none
#[test]
fn an_empty_seat_refuses_everything() {
    let mut bidet = Bidet::new(1_000, 50);
    assert_eq!(bidet.wash(Spray::Rear, 5), Err(BidetError::SeatEmpty));
    assert_eq!(bidet.dry(3), Err(BidetError::SeatEmpty));
    assert_eq!(bidet.stand(), Err(BidetError::SeatEmpty));
    assert_eq!(bidet.tank(), 1_000);
    assert_eq!(bidet.mode(), Mode::Idle);
}

/// Description: A wash takes pressure × 20 milliliters for every second from the tank up front.
/// Inputs: none
/// Outputs: none
#[test]
fn a_wash_takes_water_by_pressure_and_time() {
    let mut bidet = Bidet::new(1_000, 50);
    bidet.sit().unwrap();
    bidet.set_pressure(3);
    assert_eq!(bidet.wash(Spray::Front, 5), Ok(()));
    assert_eq!(bidet.tank(), 700);
    assert_eq!(bidet.water_used(), 300);
    assert_eq!(bidet.mode(), Mode::Washing { spray: Spray::Front, seconds_left: 5 });
}

/// Description: The pressure starts at the lowest level, and a level outside the range is pulled
/// back to the nearest end.
/// Inputs: none
/// Outputs: none
#[test]
fn the_pressure_stays_in_range() {
    let mut bidet = Bidet::new(1_000, 50);
    assert_eq!(bidet.pressure(), MIN_PRESSURE);
    bidet.set_pressure(9);
    assert_eq!(bidet.pressure(), MAX_PRESSURE);
    bidet.set_pressure(0);
    assert_eq!(bidet.pressure(), MIN_PRESSURE);
}

/// Description: A wash that needs more water than the tank holds is refused and changes nothing.
/// Inputs: none
/// Outputs: none
#[test]
fn a_low_tank_refuses_the_wash() {
    let mut bidet = Bidet::new(200, 50);
    bidet.sit().unwrap();
    bidet.set_pressure(5);
    assert_eq!(
        bidet.wash(Spray::Rear, 3),
        Err(BidetError::TankLow { needed: 300, available: 200 })
    );
    assert_eq!(bidet.tank(), 200);
    assert_eq!(bidet.water_used(), 0);
    assert_eq!(bidet.mode(), Mode::Idle);
}

/// Description: While a wash runs, a second wash or a dry is refused.
/// Inputs: none
/// Outputs: none
#[test]
fn a_busy_bidet_refuses_new_work() {
    let mut bidet = Bidet::new(1_000, 50);
    bidet.sit().unwrap();
    bidet.wash(Spray::Rear, 2).unwrap();
    assert_eq!(bidet.dry(3), Err(BidetError::Busy));
    assert_eq!(bidet.wash(Spray::Front, 1), Err(BidetError::Busy));
}

/// Description: A wash counts down one second per tick and returns to idle when its time is up,
/// and a dry can start after it.
/// Inputs: none
/// Outputs: none
#[test]
fn ticks_count_down_back_to_idle() {
    let mut bidet = Bidet::new(1_000, 50);
    bidet.sit().unwrap();
    bidet.wash(Spray::Rear, 2).unwrap();
    bidet.tick();
    assert_eq!(bidet.mode(), Mode::Washing { spray: Spray::Rear, seconds_left: 1 });
    bidet.tick();
    assert_eq!(bidet.mode(), Mode::Idle);
    assert_eq!(bidet.dry(1), Ok(()));
    bidet.tick();
    assert_eq!(bidet.mode(), Mode::Idle);
}

/// Description: Each tick refills the tank, but never past a full tank.
/// Inputs: none
/// Outputs: none
#[test]
fn the_refill_stops_at_a_full_tank() {
    let mut bidet = Bidet::new(1_000, 50);
    bidet.sit().unwrap();
    bidet.wash(Spray::Rear, 10).unwrap();
    assert_eq!(bidet.tank(), 800);
    for _ in 0..3 {
        bidet.tick();
    }
    assert_eq!(bidet.tank(), 950);
    for _ in 0..2 {
        bidet.tick();
    }
    assert_eq!(bidet.tank(), 1_000);
}

/// Description: Standing up stops a running wash and flushes, and the water already taken stays
/// used. The stop button also ends a dry.
/// Inputs: none
/// Outputs: none
#[test]
fn standing_up_stops_the_bidet_and_flushes() {
    let mut bidet = Bidet::new(1_000, 50);
    bidet.sit().unwrap();
    bidet.wash(Spray::Rear, 5).unwrap();
    assert_eq!(bidet.stand(), Ok(()));
    assert_eq!(bidet.mode(), Mode::Idle);
    assert_eq!(bidet.flushes(), 1);
    assert_eq!(bidet.tank(), 900);
    assert_eq!(bidet.wash(Spray::Rear, 5), Err(BidetError::SeatEmpty));

    bidet.sit().unwrap();
    bidet.dry(10).unwrap();
    bidet.stop();
    assert_eq!(bidet.mode(), Mode::Idle);
}
