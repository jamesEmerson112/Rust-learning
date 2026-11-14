//! Simulates a self-cleaning litter box that runs a clean cycle after a cat leaves.
//!
//! APIs:
//! - `CAT_GRAMS`: the lightest reading that counts as a cat
//! - `CYCLE_MINUTES`: how long one clean cycle takes
//! - `State`: what the box is doing
//! - `LitterError`: why the cycle button was refused
//! - `LitterBox`: the sensor state, the timers and the waste drawer
//! - `LitterBox::new`: a ready box with an empty drawer
//! - `LitterBox::state`: what the box is doing now
//! - `LitterBox::drawer_level`: the clean cycles since the drawer was emptied
//! - `LitterBox::drawer_full`: whether the drawer needs emptying
//! - `LitterBox::visits`: the cat weight recorded at each visit
//! - `LitterBox::sense_weight`: feed one weight-sensor reading
//! - `LitterBox::tick`: advance the timers by one minute
//! - `LitterBox::clean_now`: the owner presses the cycle button
//! - `LitterBox::empty_drawer`: the owner empties the waste drawer

/// Description: The lightest weight-sensor reading, in grams, that counts as a cat.
pub const CAT_GRAMS: u32 = 1_000;

/// Description: How many minutes one clean cycle takes.
pub const CYCLE_MINUTES: u32 = 2;

/// Description: What the litter box is doing.
/// Variants:
/// - `Ready`: idle and clean, waiting for a cat.
/// - `CatInside`: the sensor reads a cat, so nothing moves.
/// - `Waiting`: the cat has left, and a clean cycle starts when `minutes_left` runs out.
/// - `Cleaning`: the globe is turning, with `minutes_left` until the cycle ends.
/// - `Paused`: a cat stepped in mid-cycle, which froze the cycle with `minutes_left` to go.
/// - `DrawerFull`: idle, but it will not clean until the drawer is emptied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Ready,
    CatInside,
    Waiting { minutes_left: u32 },
    Cleaning { minutes_left: u32 },
    Paused { minutes_left: u32 },
    DrawerFull,
}

/// Description: The reason the cycle button was refused.
/// Variants:
/// - `CatInside`: a cat is in the globe.
/// - `AlreadyCleaning`: a cycle is already running.
/// - `DrawerFull`: the waste drawer has no room for another cycle.
#[derive(Debug, PartialEq)]
pub enum LitterError {
    CatInside,
    AlreadyCleaning,
    DrawerFull,
}

/// Description: One self-cleaning litter box.
/// Fields:
/// - `state`: what the box is doing.
/// - `wait_minutes`: the delay between a cat leaving and the clean cycle starting.
/// - `drawer_level`: the clean cycles run since the drawer was last emptied.
/// - `drawer_capacity`: the clean cycles the drawer holds before it is full.
/// - `visits`: the cat weight in grams recorded at the start of each visit.
pub struct LitterBox {
    state:           State,
    wait_minutes:    u32,
    drawer_level:    u32,
    drawer_capacity: u32,
    visits:          Vec<u32>,
}

/// Description: Runs one cat visit through the countdown and the clean cycle, printing the state
/// each minute.
/// Inputs: none
/// Outputs: none
fn main() {
    let mut litter_box = LitterBox::new(3, 10);
    litter_box.sense_weight(4_500);
    litter_box.sense_weight(0);

    for minute in 1..=5 {
        litter_box.tick();
        // Compressed:
        // println!("minute {minute}: {:?}", litter_box.state());
        let state = litter_box.state();
        println!("minute {minute}: {state:?}");
    }
}

impl LitterBox {
    /// Description: Builds a ready box with an empty drawer.
    /// Inputs:
    /// - `wait_minutes`: the delay between a cat leaving and the clean cycle starting.
    /// - `drawer_capacity`: the clean cycles the drawer holds before it is full.
    /// Outputs: the new litter box.
    /// Constraints: panics when `drawer_capacity` is zero.
    pub fn new(wait_minutes: u32, drawer_capacity: u32) -> Self {
        assert!(drawer_capacity > 0, "the drawer must hold at least one cycle");

        LitterBox {
            state:        State::Ready,
            wait_minutes,
            drawer_level: 0,
            drawer_capacity,
            visits:       Vec::new(),
        }
    }

    /// Description: Feeds one weight-sensor reading. A cat stepping in records a visit, stops any
    /// countdown and pauses a running cycle. A cat stepping out starts the countdown, or resumes
    /// a paused cycle.
    /// Inputs:
    /// - `grams`: the weight on the sensor.
    /// Outputs: none
    /// Constraints: readings below `CAT_GRAMS` count as an empty box. Repeated readings during one
    /// visit record that visit once.
    pub fn sense_weight(&mut self, grams: u32) {
        let cat    = grams >= CAT_GRAMS;
        self.state = match (self.state, cat) {
            (State::Ready | State::Waiting { .. } | State::DrawerFull, true) => {
                self.visits.push(grams);
                State::CatInside
            }
            (State::Cleaning { minutes_left }, true) => {
                self.visits.push(grams);
                State::Paused { minutes_left }
            }
            (State::CatInside, false) if self.drawer_full() => State::DrawerFull,
            (State::CatInside, false) => State::Waiting { minutes_left: self.wait_minutes },
            (State::Paused { minutes_left }, false) => State::Cleaning { minutes_left },
            (state, _) => state,
        };
    }

    /// Description: Advances the countdown or the running cycle by one minute. A finished cycle
    /// adds one load to the drawer.
    /// Inputs: none
    /// Outputs: none
    pub fn tick(&mut self) {
        self.state = match self.state {
            State::Waiting { minutes_left } if minutes_left > 1 => {
                // Compressed:
                // State::Waiting { minutes_left: minutes_left - 1 }
                let minutes_after_tick = minutes_left - 1;
                State::Waiting { minutes_left: minutes_after_tick }
            }
            State::Waiting { .. } => State::Cleaning { minutes_left: CYCLE_MINUTES },
            State::Cleaning { minutes_left } if minutes_left > 1 => {
                // Compressed:
                // State::Cleaning { minutes_left: minutes_left - 1 }
                let minutes_after_tick = minutes_left - 1;
                State::Cleaning { minutes_left: minutes_after_tick }
            }
            State::Cleaning { .. } => {
                self.drawer_level += 1;
                if self.drawer_full() {
                    State::DrawerFull
                } else {
                    State::Ready
                }
            }
            state => state,
        };
    }

    /// Description: Starts a clean cycle at once, skipping any countdown.
    /// Inputs: none
    /// Outputs: `Ok(())` once the cycle starts, or `CatInside`, `AlreadyCleaning` or
    /// `DrawerFull`.
    pub fn clean_now(&mut self) -> Result<(), LitterError> {
        match self.state {
            State::Ready | State::Waiting { .. } => {
                self.state = State::Cleaning { minutes_left: CYCLE_MINUTES };
                Ok(())
            }
            State::CatInside | State::Paused { .. } => Err(LitterError::CatInside),
            State::Cleaning { .. } => Err(LitterError::AlreadyCleaning),
            State::DrawerFull => Err(LitterError::DrawerFull),
        }
    }

    /// Description: Empties the waste drawer, which lets a full box clean again.
    /// Inputs: none
    /// Outputs: none
    pub fn empty_drawer(&mut self) {
        self.drawer_level = 0;
        if self.state == State::DrawerFull {
            self.state = State::Ready;
        }
    }

    /// Description: Reports whether the drawer needs emptying.
    /// Inputs: none
    /// Outputs: `true` once the drawer holds `drawer_capacity` cycles.
    pub fn drawer_full(&self) -> bool {
        self.drawer_level >= self.drawer_capacity
    }

    /// Description: Returns what the box is doing now.
    /// Inputs: none
    /// Outputs: the current state.
    pub fn state(&self) -> State {
        self.state
    }

    /// Description: Returns how many clean cycles have run since the drawer was emptied.
    /// Inputs: none
    /// Outputs: the drawer level in cycles.
    pub fn drawer_level(&self) -> u32 {
        self.drawer_level
    }

    /// Description: Returns the visit log.
    /// Inputs: none
    /// Outputs: the cat weight in grams recorded at each visit, oldest first.
    pub fn visits(&self) -> &[u32] {
        &self.visits
    }
}
