//! Simulates a dual-flush toilet stall with a door lock and a refilling tank.
//!
//! APIs:
//! - `Stall`: whether the stall is free, in use or out of order
//! - `Flush`: the two flush sizes
//! - `Flush::liters`: the water one flush uses
//! - `ToiletError`: why an operation was refused
//! - `Toilet`: the stall, its tank and its water meter
//! - `Toilet::new`: a vacant stall with a full tank
//! - `Toilet::stall`: the stall state
//! - `Toilet::tank`: the liters in the tank
//! - `Toilet::liters_used`: the total liters flushed
//! - `Toilet::lock`: a visitor takes the stall
//! - `Toilet::unlock`: the visitor leaves
//! - `Toilet::flush`: flush with a chosen size
//! - `Toilet::tick`: refill the tank by one tick's worth
//! - `Toilet::report_clog`: take the stall out of order
//! - `Toilet::repair`: put an out-of-order stall back in service

/// Description: The state of the stall.
/// Variants:
/// - `Vacant`: free to use.
/// - `Occupied`: locked by a visitor.
/// - `OutOfOrder`: clogged and closed until it is repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stall {
    Vacant,
    Occupied,
    OutOfOrder,
}

/// Description: The two flush sizes of a dual-flush toilet.
/// Variants:
/// - `Half`: the small flush.
/// - `Full`: the large flush.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flush {
    Half,
    Full,
}

impl Flush {
    /// Description: Returns how much water this flush uses.
    /// Inputs: none
    /// Outputs: the flush volume in liters, which is 3 for `Half` and 6 for `Full`.
    pub fn liters(self) -> u32 {
        match self {
            Flush::Half => 3,
            Flush::Full => 6,
        }
    }
}

/// Description: The reason the toilet refused an operation.
/// Variants:
/// - `AlreadyOccupied`: someone has already locked the stall.
/// - `NotOccupied`: nobody is inside to unlock it.
/// - `OutOfOrder`: the stall is closed until it is repaired.
/// - `TankLow`: the flush `needed` more liters than the tank had `available`.
#[derive(Debug, PartialEq)]
pub enum ToiletError {
    AlreadyOccupied,
    NotOccupied,
    OutOfOrder,
    TankLow { needed: u32, available: u32 },
}

/// Description: One toilet stall with its tank and a meter of the water it has used.
/// Fields:
/// - `stall`: the state of the stall.
/// - `tank`: the liters in the tank now.
/// - `capacity`: the liters in a full tank.
/// - `refill_per_tick`: the liters the refill valve adds on each tick.
/// - `liters_used`: the total liters flushed so far.
pub struct Toilet {
    stall: Stall,
    tank: u32,
    capacity: u32,
    refill_per_tick: u32,
    liters_used: u32,
}

impl Toilet {
    /// Description: Builds a vacant stall with a full tank.
    /// Inputs:
    /// - `capacity`: the liters in a full tank.
    /// - `refill_per_tick`: the liters the refill valve adds on each tick.
    /// Outputs: the new toilet.
    pub fn new(capacity: u32, refill_per_tick: u32) -> Self {
        Toilet { stall: Stall::Vacant, tank: capacity, capacity, refill_per_tick, liters_used: 0 }
    }

    /// Description: Returns the state of the stall.
    /// Inputs: none
    /// Outputs: vacant, occupied or out of order.
    pub fn stall(&self) -> Stall {
        self.stall
    }

    /// Description: Returns the water in the tank.
    /// Inputs: none
    /// Outputs: the liters in the tank now.
    pub fn tank(&self) -> u32 {
        self.tank
    }

    /// Description: Returns the water flushed so far.
    /// Inputs: none
    /// Outputs: the total liters flushed.
    pub fn liters_used(&self) -> u32 {
        self.liters_used
    }

    /// Description: Locks the stall for a visitor.
    /// Inputs: none
    /// Outputs: `Ok(())` once locked, or `AlreadyOccupied` or `OutOfOrder` when the stall cannot
    /// be taken.
    pub fn lock(&mut self) -> Result<(), ToiletError> {
        match self.stall {
            Stall::Vacant => {
                self.stall = Stall::Occupied;
                Ok(())
            }
            Stall::Occupied => Err(ToiletError::AlreadyOccupied),
            Stall::OutOfOrder => Err(ToiletError::OutOfOrder),
        }
    }

    /// Description: Unlocks the stall when the visitor leaves.
    /// Inputs: none
    /// Outputs: `Ok(())` once unlocked, or `NotOccupied` or `OutOfOrder` when nobody can leave.
    pub fn unlock(&mut self) -> Result<(), ToiletError> {
        match self.stall {
            Stall::Occupied => {
                self.stall = Stall::Vacant;
                Ok(())
            }
            Stall::Vacant => Err(ToiletError::NotOccupied),
            Stall::OutOfOrder => Err(ToiletError::OutOfOrder),
        }
    }

    /// Description: Flushes with the chosen size and takes that much water from the tank.
    /// Inputs:
    /// - `kind`: the flush size.
    /// Outputs: the liters left in the tank, or `OutOfOrder`, or `TankLow` when the tank holds
    /// less water than the flush needs.
    /// Constraints: a refused flush leaves the tank and the meter unchanged.
    pub fn flush(&mut self, kind: Flush) -> Result<u32, ToiletError> {
        if self.stall == Stall::OutOfOrder {
            return Err(ToiletError::OutOfOrder);
        }
        let needed = kind.liters();
        if self.tank < needed {
            return Err(ToiletError::TankLow { needed, available: self.tank });
        }
        self.tank -= needed;
        self.liters_used += needed;
        Ok(self.tank)
    }

    /// Description: Lets the refill valve run for one tick.
    /// Inputs: none
    /// Outputs: none
    /// Constraints: the tank never rises above `capacity`.
    pub fn tick(&mut self) {
        self.tank = self.tank.saturating_add(self.refill_per_tick).min(self.capacity);
    }

    /// Description: Marks the toilet clogged, which closes the stall even if someone is inside.
    /// Inputs: none
    /// Outputs: none
    pub fn report_clog(&mut self) {
        self.stall = Stall::OutOfOrder;
    }

    /// Description: Puts an out-of-order stall back in service. It does nothing in any other
    /// state.
    /// Inputs: none
    /// Outputs: none
    pub fn repair(&mut self) {
        if self.stall == Stall::OutOfOrder {
            self.stall = Stall::Vacant;
        }
    }
}

/// Description: Runs one visit with two full flushes and a refill, printing the result of each.
/// Inputs: none
/// Outputs: none
fn main() {
    let mut toilet = Toilet::new(9, 2);
    toilet.lock().unwrap();
    println!("first full flush: {:?}", toilet.flush(Flush::Full));
    println!("second full flush: {:?}", toilet.flush(Flush::Full));
    toilet.tick();
    toilet.unlock().unwrap();
    println!("after one tick the tank holds {} liters", toilet.tank());
}
