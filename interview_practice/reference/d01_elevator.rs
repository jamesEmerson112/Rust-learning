//! Simulates one elevator car that serves floor requests in sweep order.
//!
//! APIs:
//! - `Direction`: which way the car is travelling
//! - `ElevatorError`: why a request was rejected
//! - `Elevator`: the car, its position and its queued stops
//! - `Elevator::new`: a car parked at the ground floor
//! - `Elevator::floor`: the current floor
//! - `Elevator::direction`: the current direction
//! - `Elevator::request`: queue a stop
//! - `Elevator::next_direction`: the sweep rule for the next move
//! - `Elevator::step`: advance the simulation by one tick
//! - `Elevator::run_until_idle`: serve every queued stop

use std::collections::BTreeSet;

/// Description: The way the car is travelling.
/// Variants:
/// - `Up`: moving toward higher floors.
/// - `Down`: moving toward lower floors.
/// - `Idle`: parked with nothing to serve.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Idle,
}

/// Description: The reason a floor request was rejected.
/// Variants:
/// - `FloorOutOfRange`: the requested `floor` is above the building's `top` floor.
#[derive(Debug, PartialEq)]
pub enum ElevatorError {
    FloorOutOfRange { floor: u32, top: u32 },
}

/// Description: One elevator car and the stops it still has to serve.
/// Fields:
/// - `floor`: the floor the car is at, where 0 is the ground floor.
/// - `top_floor`: the highest floor in the building.
/// - `direction`: the way the car is travelling.
/// - `stops`: the floors still to serve, kept sorted and free of duplicates.
pub struct Elevator {
    floor: u32,
    top_floor: u32,
    direction: Direction,
    stops: BTreeSet<u32>,
}

impl Elevator {
    /// Description: Builds a car parked at the ground floor with no stops queued.
    /// Inputs:
    /// - `top_floor`: the highest floor in the building.
    /// Outputs: the new car.
    pub fn new(top_floor: u32) -> Self {
        Elevator { floor: 0, top_floor, direction: Direction::Idle, stops: BTreeSet::new() }
    }

    /// Description: Returns the current floor.
    /// Inputs: none
    /// Outputs: the floor the car is at.
    pub fn floor(&self) -> u32 {
        self.floor
    }

    /// Description: Returns the current direction.
    /// Inputs: none
    /// Outputs: the way the car is travelling.
    pub fn direction(&self) -> Direction {
        self.direction
    }

    /// Description: Queues a stop at a floor. Pressing a floor that is already queued changes
    /// nothing.
    /// Inputs:
    /// - `floor`: the floor to stop at.
    /// Outputs: `Ok(())` once the stop is queued, or `FloorOutOfRange` for a floor above the top
    /// floor.
    pub fn request(&mut self, floor: u32) -> Result<(), ElevatorError> {
        if floor > self.top_floor {
            return Err(ElevatorError::FloorOutOfRange { floor, top: self.top_floor });
        }
        self.stops.insert(floor);
        Ok(())
    }

    /// Description: Chooses the direction of the next move by the LOOK rule. The car keeps its
    /// direction while a stop remains ahead, and otherwise heads toward whatever stop is left.
    /// Inputs: none
    /// Outputs: `Up` or `Down`, or `Idle` when no stops remain.
    pub fn next_direction(&self) -> Direction {
        let above = self.stops.range(self.floor + 1..).next().is_some();
        let below = self.stops.range(..self.floor).next().is_some();
        match self.direction {
            Direction::Up if above => Direction::Up,
            Direction::Down if below => Direction::Down,
            _ if above => Direction::Up,
            _ if below => Direction::Down,
            _ => Direction::Idle,
        }
    }

    /// Description: Advances the simulation by one tick. When the current floor is a stop, the
    /// doors open without moving. Otherwise the car moves one floor and opens if it reaches a
    /// stop. The car goes idle once no stops remain.
    /// Inputs: none
    /// Outputs: `Some(floor)` when the doors open on this tick, or `None` when they stay shut.
    pub fn step(&mut self) -> Option<u32> {
        if !self.stops.remove(&self.floor) {
            self.direction = self.next_direction();
            match self.direction {
                Direction::Up => self.floor += 1,
                Direction::Down => self.floor -= 1,
                Direction::Idle => return None,
            }
            if !self.stops.remove(&self.floor) {
                return None;
            }
        }
        if self.stops.is_empty() {
            self.direction = Direction::Idle;
        }
        Some(self.floor)
    }

    /// Description: Steps the car until every queued stop has been served.
    /// Inputs: none
    /// Outputs: the floors where the doors opened, in the order they opened.
    /// Constraints: gives up after 4 × (`top_floor` + 1) ticks, so a broken `step` fails a test
    /// instead of hanging it.
    pub fn run_until_idle(&mut self) -> Vec<u32> {
        let mut opened = Vec::new();
        let max_steps = 4 * (self.top_floor as usize + 1);
        for _ in 0..max_steps {
            if self.stops.is_empty() {
                break;
            }
            if let Some(floor) = self.step() {
                opened.push(floor);
            }
        }
        opened
    }
}

/// Description: Queues three floors out of order and prints the order the car serves them in.
/// Inputs: none
/// Outputs: none
fn main() {
    let mut car = Elevator::new(10);
    for floor in [7, 3, 5] {
        car.request(floor).unwrap();
    }
    println!("doors opened at {:?}", car.run_until_idle());
}
