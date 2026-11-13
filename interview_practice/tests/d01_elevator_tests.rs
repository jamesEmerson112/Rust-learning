//! Tests for the elevator car in `src/bin/d01_elevator.rs`.
//!
//! Contents:
//! - `request` tests: the range check and duplicate presses
//! - `next_direction` tests: the LOOK sweep rule
//! - `step` tests: one floor per tick, opening the doors and going idle
//! - whole-trip tests: the order the doors open in

#[path = "../src/bin/d01_elevator.rs"]
#[allow(dead_code)]
mod d01_elevator;

use d01_elevator::{Direction, Elevator, ElevatorError};

/// Description: A floor above the top is rejected, and the top floor itself is accepted.
/// Inputs: none
/// Outputs: none
#[test]
fn rejects_a_floor_above_the_top() {
    let mut car = Elevator::new(10);
    assert_eq!(car.request(11), Err(ElevatorError::FloorOutOfRange { floor: 11, top: 10 }));
    assert_eq!(car.request(10), Ok(()));
}

/// Description: Pressing the same floor twice makes the doors open there only once.
/// Inputs: none
/// Outputs: none
#[test]
fn pressing_a_floor_twice_queues_it_once() {
    let mut car = Elevator::new(10);
    car.request(4).unwrap();
    car.request(4).unwrap();
    assert_eq!(car.run_until_idle(), vec![4]);
}

/// Description: A car with nothing queued stays idle.
/// Inputs: none
/// Outputs: none
#[test]
fn idle_with_nothing_queued() {
    assert_eq!(Elevator::new(10).next_direction(), Direction::Idle);
}

/// Description: A parked car heads up toward a stop above it and down toward a stop below it.
/// Inputs: none
/// Outputs: none
#[test]
fn heads_toward_the_only_stop() {
    let mut car = Elevator::new(10);
    car.request(6).unwrap();
    assert_eq!(car.next_direction(), Direction::Up);

    car.run_until_idle();
    car.request(2).unwrap();
    assert_eq!(car.next_direction(), Direction::Down);
}

/// Description: A car at floor 1 heading up to floor 8 keeps going up after floor 0 is pressed
/// behind it.
/// Inputs: none
/// Outputs: none
#[test]
fn keeps_going_up_while_a_stop_remains_above() {
    let mut car = Elevator::new(10);
    car.request(8).unwrap();
    car.step();
    car.request(0).unwrap();
    assert_eq!(car.next_direction(), Direction::Up);
}

/// Description: A car at floor 8 heading down to floor 5 keeps going down after floor 10 is
/// pressed behind it.
/// Inputs: none
/// Outputs: none
#[test]
fn keeps_going_down_while_a_stop_remains_below() {
    let mut car = Elevator::new(10);
    car.request(9).unwrap();
    car.run_until_idle();
    car.request(5).unwrap();
    car.step();
    car.request(10).unwrap();
    assert_eq!(car.next_direction(), Direction::Down);
}

/// Description: A car that reaches floor 2 heading up turns down for the stop it passed at
/// floor 0.
/// Inputs: none
/// Outputs: none
#[test]
fn turns_around_when_nothing_is_ahead() {
    let mut car = Elevator::new(10);
    car.request(2).unwrap();
    car.step();
    car.request(0).unwrap();
    assert_eq!(car.step(), Some(2));
    assert_eq!(car.next_direction(), Direction::Down);
}

/// Description: The car climbs one floor per tick and opens its doors when it arrives.
/// Inputs: none
/// Outputs: none
#[test]
fn moves_one_floor_per_step() {
    let mut car = Elevator::new(10);
    car.request(2).unwrap();
    assert_eq!(car.step(), None);
    assert_eq!(car.floor(), 1);
    assert_eq!(car.direction(), Direction::Up);
    assert_eq!(car.step(), Some(2));
    assert_eq!(car.floor(), 2);
}

/// Description: After its last stop the car goes idle and stays where it is.
/// Inputs: none
/// Outputs: none
#[test]
fn goes_idle_after_the_last_stop() {
    let mut car = Elevator::new(10);
    car.request(1).unwrap();
    assert_eq!(car.step(), Some(1));
    assert_eq!(car.direction(), Direction::Idle);
    assert_eq!(car.step(), None);
    assert_eq!(car.floor(), 1);
}

/// Description: Pressing the floor the car is already at opens the doors without moving.
/// Inputs: none
/// Outputs: none
#[test]
fn a_request_at_the_current_floor_opens_without_moving() {
    let mut car = Elevator::new(10);
    car.request(0).unwrap();
    assert_eq!(car.step(), Some(0));
    assert_eq!(car.floor(), 0);
}

/// Description: Floors pressed as 7, 3 and 5 are served bottom to top.
/// Inputs: none
/// Outputs: none
#[test]
fn serves_stops_in_floor_order_not_press_order() {
    let mut car = Elevator::new(10);
    for floor in [7, 3, 5] {
        car.request(floor).unwrap();
    }
    assert_eq!(car.run_until_idle(), vec![3, 5, 7]);
}

/// Description: A car climbing past floor 5 toward floors 6 and 8 serves both before coming back
/// down for floor 2.
/// Inputs: none
/// Outputs: none
#[test]
fn finishes_the_up_sweep_before_coming_back_down() {
    let mut car = Elevator::new(10);
    car.request(6).unwrap();
    car.request(8).unwrap();
    for _ in 0..5 {
        car.step();
    }
    car.request(2).unwrap();
    assert_eq!(car.run_until_idle(), vec![6, 8, 2]);
    assert_eq!(car.direction(), Direction::Idle);
}
