//! Tests for the self-cleaning litter box in `src/bin/d06_litter_robot.rs`.
//!
//! Contents:
//! - `run`: a helper that ticks the box for some minutes
//! - sensor tests: what counts as a cat and how visits are recorded
//! - timer tests: the countdown, the clean cycle and the safety pause
//! - drawer test: a full drawer blocks cleaning until it is emptied
//! - button test: the cycle button and its safety checks

#[path = "../src/bin/d06_litter_robot.rs"]
#[allow(dead_code)]
mod d06_litter_robot;

use d06_litter_robot::{CYCLE_MINUTES, LitterBox, LitterError, State};

/// Description: Ticks the box once per minute.
/// Inputs:
/// - `litter_box`: the box to advance.
/// - `minutes`: how many ticks to run.
/// Outputs: none
fn run(litter_box: &mut LitterBox, minutes: u32) {
    for _ in 0..minutes {
        litter_box.tick();
    }
}

/// Description: A new box is ready, its drawer is empty and it has seen no cats.
/// Inputs: none
/// Outputs: none
#[test]
fn a_new_box_is_ready_with_an_empty_drawer() {
    let litter_box = LitterBox::new(3, 5);
    assert_eq!(litter_box.state(), State::Ready);
    assert_eq!(litter_box.drawer_level(), 0);
    assert!(!litter_box.drawer_full());
    assert!(litter_box.visits().is_empty());
}

/// Description: A reading lighter than a cat, such as shifting litter, changes nothing.
/// Inputs: none
/// Outputs: none
#[test]
fn a_light_reading_is_not_a_cat() {
    let mut litter_box = LitterBox::new(3, 5);
    litter_box.sense_weight(400);
    assert_eq!(litter_box.state(), State::Ready);
    assert!(litter_box.visits().is_empty());
}

/// Description: Several readings during one visit record the visit once, with the first weight.
/// Inputs: none
/// Outputs: none
#[test]
fn one_visit_is_recorded_once() {
    let mut litter_box = LitterBox::new(3, 5);
    for grams in [4_500, 4_550, 4_480] {
        litter_box.sense_weight(grams);
    }
    assert_eq!(litter_box.visits(), &[4_500]);
}

/// Description: After the cat leaves, the box waits 3 minutes, cleans for 2 and is then ready
/// with one load in the drawer.
/// Inputs: none
/// Outputs: none
#[test]
fn a_visit_starts_the_countdown_and_then_a_clean_cycle() {
    let mut litter_box = LitterBox::new(3, 5);
    litter_box.sense_weight(4_500);
    assert_eq!(litter_box.state(), State::CatInside);
    litter_box.sense_weight(0);
    assert_eq!(litter_box.state(), State::Waiting { minutes_left: 3 });
    run(&mut litter_box, 2);
    assert_eq!(litter_box.state(), State::Waiting { minutes_left: 1 });
    run(&mut litter_box, 1);
    assert_eq!(litter_box.state(), State::Cleaning { minutes_left: CYCLE_MINUTES });
    run(&mut litter_box, CYCLE_MINUTES);
    assert_eq!(litter_box.state(), State::Ready);
    assert_eq!(litter_box.drawer_level(), 1);
}

/// Description: A cat that comes back during the countdown restarts it from the full delay.
/// Inputs: none
/// Outputs: none
#[test]
fn a_returning_cat_restarts_the_countdown() {
    let mut litter_box = LitterBox::new(3, 5);
    litter_box.sense_weight(4_500);
    litter_box.sense_weight(0);
    run(&mut litter_box, 2);
    litter_box.sense_weight(4_600);
    assert_eq!(litter_box.state(), State::CatInside);
    litter_box.sense_weight(0);
    assert_eq!(litter_box.state(), State::Waiting { minutes_left: 3 });
    assert_eq!(litter_box.visits(), &[4_500, 4_600]);
}

/// Description: A cat stepping in mid-cycle freezes the cycle, and the cycle resumes where it
/// stopped once the cat leaves.
/// Inputs: none
/// Outputs: none
#[test]
fn a_cat_entering_mid_cycle_pauses_it() {
    let mut litter_box = LitterBox::new(3, 5);
    litter_box.clean_now().unwrap();
    run(&mut litter_box, 1);
    litter_box.sense_weight(5_000);
    assert_eq!(litter_box.state(), State::Paused { minutes_left: 1 });
    run(&mut litter_box, 10);
    assert_eq!(litter_box.state(), State::Paused { minutes_left: 1 });
    litter_box.sense_weight(0);
    assert_eq!(litter_box.state(), State::Cleaning { minutes_left: 1 });
    run(&mut litter_box, 1);
    assert_eq!(litter_box.state(), State::Ready);
    assert_eq!(litter_box.drawer_level(), 1);
}

/// Description: A drawer that holds two cycles is full after two cycles. The box then refuses to
/// clean and skips the countdown after a visit, and it recovers once the drawer is emptied.
/// Inputs: none
/// Outputs: none
#[test]
fn a_full_drawer_blocks_cleaning_until_emptied() {
    let mut litter_box = LitterBox::new(3, 2);
    for _ in 0..2 {
        litter_box.clean_now().unwrap();
        run(&mut litter_box, CYCLE_MINUTES);
    }
    assert_eq!(litter_box.state(), State::DrawerFull);
    assert!(litter_box.drawer_full());
    assert_eq!(litter_box.clean_now(), Err(LitterError::DrawerFull));

    litter_box.sense_weight(4_500);
    litter_box.sense_weight(0);
    assert_eq!(litter_box.state(), State::DrawerFull);

    litter_box.empty_drawer();
    assert_eq!(litter_box.state(), State::Ready);
    assert_eq!(litter_box.clean_now(), Ok(()));
}

/// Description: The cycle button skips the countdown, but it refuses while a cat is inside or a
/// cycle is already running.
/// Inputs: none
/// Outputs: none
#[test]
fn the_cycle_button_respects_the_cat_and_a_running_cycle() {
    let mut litter_box = LitterBox::new(3, 5);
    litter_box.sense_weight(4_500);
    assert_eq!(litter_box.clean_now(), Err(LitterError::CatInside));
    litter_box.sense_weight(0);
    assert_eq!(litter_box.clean_now(), Ok(()));
    assert_eq!(litter_box.state(), State::Cleaning { minutes_left: CYCLE_MINUTES });
    assert_eq!(litter_box.clean_now(), Err(LitterError::AlreadyCleaning));
}
