//! Simulates a smart bidet toilet seat with a seat sensor, a warm-water wash, a dryer and an
//! automatic flush.
//!
//! APIs:
//! - `MIN_PRESSURE`, `MAX_PRESSURE`: the range of spray pressure levels
//! - `ML_PER_LEVEL_SECOND`: the water one pressure level sprays each second
//! - `Spray`: which nozzle washes
//! - `Mode`: what the bidet is doing now
//! - `BidetError`: why a request was refused
//! - `Bidet`: the seat, its warm-water tank and its counters
//! - `Bidet::new`: an empty seat with a full tank
//! - `Bidet::sit`: the seat sensor feels someone sit down
//! - `Bidet::set_pressure`: choose the spray pressure
//! - `Bidet::wash`: start a timed wash
//! - `Bidet::tick`: advance one second
//! - `Bidet::dry`: start a timed warm-air dry
//! - `Bidet::stand`: the user stands up, which stops the bidet and flushes
//! - `Bidet::stop`: the stop button on the remote
//! - `Bidet::mode`, `Bidet::pressure`, `Bidet::tank`, `Bidet::water_used`, `Bidet::flushes`:
//!   the readings

use std::cmp::min;

/// Description: The lowest spray pressure level.
pub const MIN_PRESSURE: u32 = 1;

/// Description: The highest spray pressure level.
pub const MAX_PRESSURE: u32 = 5;

/// Description: The milliliters of warm water that one pressure level sprays each second.
pub const ML_PER_LEVEL_SECOND: u32 = 20;

/// Description: The nozzle that washes.
/// Variants:
/// - `Rear`: the rear wash.
/// - `Front`: the front wash.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spray {
    Rear,
    Front,
}

/// Description: What the bidet is doing now.
/// Variants:
/// - `Idle`: waiting, with the nozzle and the dryer off.
/// - `Washing`: spraying from the `spray` nozzle for `seconds_left` more seconds.
/// - `Drying`: blowing warm air for `seconds_left` more seconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Idle,
    Washing { spray: Spray, seconds_left: u32 },
    Drying { seconds_left: u32 },
}

/// Description: The reason the bidet refused a request.
/// Variants:
/// - `SeatEmpty`: nobody is seated, so the bidet will not spray, dry or register a stand.
/// - `AlreadySeated`: the seat sensor already reports someone seated.
/// - `Busy`: a wash or a dry is still running.
/// - `TankLow`: the wash `needed` more milliliters than the tank had `available`.
#[derive(Debug, PartialEq)]
pub enum BidetError {
    SeatEmpty,
    AlreadySeated,
    Busy,
    TankLow { needed: u32, available: u32 },
}

/// Description: One smart bidet seat with its warm-water tank and its usage counters.
/// Fields:
/// - `seated`: whether the seat sensor feels someone sitting.
/// - `mode`: what the bidet is doing now.
/// - `pressure`: the spray pressure level, from `MIN_PRESSURE` to `MAX_PRESSURE`.
/// - `tank`: the milliliters of warm water in the tank now.
/// - `capacity`: the milliliters in a full tank.
/// - `refill_per_tick`: the milliliters the heater refills each second.
/// - `water_used`: the total milliliters sprayed so far.
/// - `flushes`: how many times the toilet has flushed.
pub struct Bidet {
    seated:          bool,
    mode:            Mode,
    pressure:        u32,
    tank:            u32,
    capacity:        u32,
    refill_per_tick: u32,
    water_used:      u32,
    flushes:         u32,
}

/// Description: Runs one visit with a five-second rear wash at pressure 3 and a three-second dry,
/// then prints what the bidet reports after the user stands up.
/// Inputs: none
/// Outputs: none
fn main() {
    let mut bidet = Bidet::new(1_000, 50);
    bidet.sit().unwrap();
    bidet.set_pressure(3);

    bidet.wash(Spray::Rear, 5).unwrap();
    for _ in 0..5 {
        bidet.tick();
    }

    bidet.dry(3).unwrap();
    for _ in 0..3 {
        bidet.tick();
    }

    bidet.stand().unwrap();

    // Compressed:
    // println!(
    //     "mode {:?}, tank {} ml, sprayed {} ml, flushes {}",
    //     bidet.mode(),
    //     bidet.tank(),
    //     bidet.water_used(),
    //     bidet.flushes()
    // );
    let mode       = bidet.mode();
    let tank       = bidet.tank();
    let water_used = bidet.water_used();
    let flushes    = bidet.flushes();
    println!("mode {mode:?}, tank {tank} ml, sprayed {water_used} ml, flushes {flushes}");
}

impl Bidet {
    /// Description: Builds an empty seat with a full tank at the lowest pressure.
    /// Inputs:
    /// - `capacity`: the milliliters in a full tank.
    /// - `refill_per_tick`: the milliliters the heater refills each second.
    /// Outputs: the new bidet.
    pub fn new(capacity: u32, refill_per_tick: u32) -> Self {
        Bidet {
            seated:     false,
            mode:       Mode::Idle,
            pressure:   MIN_PRESSURE,
            tank:       capacity,
            capacity,
            refill_per_tick,
            water_used: 0,
            flushes:    0,
        }
    }

    /// Description: Records that someone sat down on the seat.
    /// Inputs: none
    /// Outputs: `Ok(())` once seated, or `AlreadySeated` when the sensor already reports someone.
    pub fn sit(&mut self) -> Result<(), BidetError> {
        if self.seated {
            return Err(BidetError::AlreadySeated);
        }

        self.seated = true;
        Ok(())
    }

    /// Description: Sets the spray pressure, pulling an out-of-range level back to the nearest end
    /// of the range.
    /// Inputs:
    /// - `level`: the requested pressure level.
    /// Outputs: none
    pub fn set_pressure(&mut self, level: u32) {
        self.pressure = level.clamp(MIN_PRESSURE, MAX_PRESSURE);
    }

    /// Description: Starts a timed wash and takes all the water it needs from the tank up front.
    /// Inputs:
    /// - `spray`: the nozzle to wash with.
    /// - `seconds`: how long to wash.
    /// Outputs: `Ok(())` once the wash starts, or `SeatEmpty`, `Busy` or `TankLow`.
    /// Constraints: the wash sprays `pressure` × `ML_PER_LEVEL_SECOND` milliliters each second. A
    /// refused wash leaves the tank and the counters unchanged.
    pub fn wash(&mut self, spray: Spray, seconds: u32) -> Result<(), BidetError> {
        self.ready()?;

        // Compressed:
        // let needed = (self.pressure * ML_PER_LEVEL_SECOND).saturating_mul(seconds);
        let ml_per_second = self.pressure * ML_PER_LEVEL_SECOND;
        let needed        = ml_per_second.saturating_mul(seconds);
        if self.tank < needed {
            return Err(BidetError::TankLow { needed, available: self.tank });
        }

        self.tank       -= needed;
        self.water_used += needed;
        self.mode        = Mode::Washing { spray, seconds_left: seconds };
        Ok(())
    }

    /// Description: Advances the bidet by one second. The heater refills the tank, and a running
    /// wash or dry counts down and returns to idle when its time is up.
    /// Inputs: none
    /// Outputs: none
    /// Constraints: the tank never rises above `capacity`.
    pub fn tick(&mut self) {
        // Compressed:
        // self.tank = self.tank.saturating_add(self.refill_per_tick).min(self.capacity);
        let refilled = self.tank.saturating_add(self.refill_per_tick);
        self.tank    = min(refilled, self.capacity);

        // Compressed:
        // self.mode = match self.mode {
        //     Mode::Washing { seconds_left, .. } | Mode::Drying { seconds_left }
        //         if seconds_left <= 1 =>
        //     {
        //         Mode::Idle
        //     }
        //     Mode::Washing { spray, seconds_left } => {
        //         Mode::Washing { spray, seconds_left: seconds_left - 1 }
        //     }
        //     Mode::Drying { seconds_left } => Mode::Drying { seconds_left: seconds_left - 1 },
        //     Mode::Idle => Mode::Idle,
        // };
        self.mode = match self.mode {
            Mode::Washing { spray, seconds_left } => {
                if seconds_left <= 1 {
                    Mode::Idle
                } else {
                    let next_seconds_left = seconds_left - 1;
                    Mode::Washing { spray, seconds_left: next_seconds_left }
                }
            }
            Mode::Drying { seconds_left } => {
                if seconds_left <= 1 {
                    Mode::Idle
                } else {
                    let next_seconds_left = seconds_left - 1;
                    Mode::Drying { seconds_left: next_seconds_left }
                }
            }
            Mode::Idle => Mode::Idle,
        };
    }

    /// Description: Starts a timed warm-air dry.
    /// Inputs:
    /// - `seconds`: how long to dry.
    /// Outputs: `Ok(())` once the dryer starts, or `SeatEmpty` or `Busy`.
    pub fn dry(&mut self, seconds: u32) -> Result<(), BidetError> {
        self.ready()?;

        self.mode = Mode::Drying { seconds_left: seconds };
        Ok(())
    }

    /// Description: Records that the user stood up. The bidet stops at once and the toilet
    /// flushes.
    /// Inputs: none
    /// Outputs: `Ok(())` once the user has left, or `SeatEmpty` when nobody was seated.
    pub fn stand(&mut self) -> Result<(), BidetError> {
        if !self.seated {
            return Err(BidetError::SeatEmpty);
        }

        self.seated = false;
        self.stop();
        self.flushes += 1;
        Ok(())
    }

    /// Description: Stops any wash or dry at once, like the stop button on the remote.
    /// Inputs: none
    /// Outputs: none
    /// Constraints: water already taken for a stopped wash is not returned to the tank.
    pub fn stop(&mut self) {
        self.mode = Mode::Idle;
    }

    /// Description: Checks that the bidet may start a wash or a dry.
    /// Inputs: none
    /// Outputs: `Ok(())` when someone is seated and nothing is running, or `SeatEmpty` or `Busy`.
    fn ready(&self) -> Result<(), BidetError> {
        if !self.seated {
            return Err(BidetError::SeatEmpty);
        }
        if self.mode != Mode::Idle {
            return Err(BidetError::Busy);
        }
        Ok(())
    }

    /// Description: Returns what the bidet is doing now.
    /// Inputs: none
    /// Outputs: idle, washing or drying.
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Description: Returns the spray pressure level.
    /// Inputs: none
    /// Outputs: a level from `MIN_PRESSURE` to `MAX_PRESSURE`.
    pub fn pressure(&self) -> u32 {
        self.pressure
    }

    /// Description: Returns the warm water in the tank.
    /// Inputs: none
    /// Outputs: the milliliters in the tank now.
    pub fn tank(&self) -> u32 {
        self.tank
    }

    /// Description: Returns the water sprayed so far.
    /// Inputs: none
    /// Outputs: the total milliliters sprayed.
    pub fn water_used(&self) -> u32 {
        self.water_used
    }

    /// Description: Returns how many times the toilet has flushed.
    /// Inputs: none
    /// Outputs: the flush count.
    pub fn flushes(&self) -> u32 {
        self.flushes
    }
}
