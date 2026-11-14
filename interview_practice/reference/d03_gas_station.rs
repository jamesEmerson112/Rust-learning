//! Simulates a gas station with prepaid pumps and one underground tank per fuel grade.
//!
//! APIs:
//! - `Grade`: the fuel grades on sale
//! - `StationError`: why a pump operation was refused
//! - `Receipt`: what one finished sale charged and refunded
//! - `Station`: the prices, tanks, pumps and revenue
//! - `Station::new`: a station with idle pumps and empty tanks
//! - `Station::set_price`: set a grade's price per liter
//! - `Station::deliver`: a tanker fills a grade's tank
//! - `Station::tank_level`: the liters left of a grade
//! - `Station::revenue_cents`: the money taken so far
//! - `Station::authorize`: a customer prepays at a pump
//! - `Station::pump`: dispense fuel on an authorized pump
//! - `Station::finish`: close the sale and hand back the receipt

use std::cmp::min;
use std::collections::HashMap;

/// Description: A fuel grade the station can sell.
/// Variants:
/// - `Regular`: regular unleaded.
/// - `Premium`: premium unleaded.
/// - `Diesel`: diesel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Grade {
    Regular,
    Premium,
    Diesel,
}

/// Description: The reason a pump operation was refused.
/// Variants:
/// - `NoSuchPump`: the station has no pump with this number.
/// - `PumpBusy`: the pump is already serving another customer.
/// - `NotAuthorized`: nobody has prepaid at this pump.
/// - `NoPrice`: the grade has no price, so it cannot be sold.
#[derive(Debug, PartialEq)]
pub enum StationError {
    NoSuchPump(usize),
    PumpBusy(usize),
    NotAuthorized(usize),
    NoPrice(Grade),
}

/// Description: The summary of one finished sale.
/// Fields:
/// - `grade`: the fuel sold.
/// - `liters`: the liters dispensed.
/// - `charged_cents`: what the customer paid for the fuel.
/// - `refund_cents`: the unused part of the prepayment, returned to the customer.
#[derive(Debug, PartialEq)]
pub struct Receipt {
    pub grade:         Grade,
    pub liters:        u32,
    pub charged_cents: u32,
    pub refund_cents:  u32,
}

/// Description: A sale in progress on one pump.
/// Fields:
/// - `grade`: the fuel being sold.
/// - `cents_per_liter`: the price, fixed when the pump was authorized.
/// - `prepaid_cents`: the amount the customer paid up front.
/// - `liters`: the liters dispensed so far.
#[derive(Debug, Clone, Copy)]
struct Sale {
    grade:           Grade,
    cents_per_liter: u32,
    prepaid_cents:   u32,
    liters:          u32,
}

/// Description: The whole station.
/// Fields:
/// - `prices`: the price in cents per liter of each grade on sale.
/// - `tanks`: the liters left in each grade's underground tank.
/// - `pumps`: one slot per pump, holding the sale in progress or `None` when the pump is idle.
/// - `revenue_cents`: the money taken from finished sales.
pub struct Station {
    prices:        HashMap<Grade, u32>,
    tanks:         HashMap<Grade, u32>,
    pumps:         Vec<Option<Sale>>,
    revenue_cents: u64,
}

/// Description: Sells one prepaid tank of regular fuel and prints the receipt.
/// Inputs: none
/// Outputs: none
fn main() {
    let mut station = Station::new(2);
    station.set_price(Grade::Regular, 180);
    station.deliver(Grade::Regular, 5_000);

    station.authorize(0, Grade::Regular, 4_000).unwrap();
    station.pump(0, 30).unwrap();

    // Compressed:
    // println!("{:?}", station.finish(0));
    let receipt = station.finish(0);
    println!("{receipt:?}");
}

impl Station {
    /// Description: Builds a station with idle pumps, empty tanks and no prices.
    /// Inputs:
    /// - `pump_count`: how many pumps the station has, numbered from 0.
    /// Outputs: the new station.
    pub fn new(pump_count: usize) -> Self {
        Station {
            prices:        HashMap::new(),
            tanks:         HashMap::new(),
            pumps:         vec![None; pump_count],
            revenue_cents: 0,
        }
    }

    /// Description: Sets the price of a grade for every sale authorized from now on.
    /// Inputs:
    /// - `grade`: the fuel grade.
    /// - `cents_per_liter`: the new price.
    /// Outputs: none
    /// Constraints: panics when `cents_per_liter` is zero, because `pump` divides by the price.
    pub fn set_price(&mut self, grade: Grade, cents_per_liter: u32) {
        assert!(cents_per_liter > 0, "a price must be positive");
        self.prices.insert(grade, cents_per_liter);
    }

    /// Description: Adds fuel to a grade's tank, as a tanker delivery does.
    /// Inputs:
    /// - `grade`: the fuel grade.
    /// - `liters`: the liters delivered.
    /// Outputs: none
    pub fn deliver(&mut self, grade: Grade, liters: u32) {
        // Compressed:
        // *self.tanks.entry(grade).or_insert(0) += liters;
        let tank = self.tanks.entry(grade).or_insert(0);
        *tank   += liters;
    }

    /// Description: Starts a sale on an idle pump after the customer prepays.
    /// Inputs:
    /// - `pump`: the pump number.
    /// - `grade`: the fuel the customer chose.
    /// - `prepaid_cents`: the amount paid up front.
    /// Outputs: `Ok(())` once the pump is ready, or `NoSuchPump`, `PumpBusy` or `NoPrice`.
    /// Constraints: the sale keeps the price in force now, even if the price changes later.
    pub fn authorize(
        &mut self,
        pump: usize,
        grade: Grade,
        prepaid_cents: u32,
    ) -> Result<(), StationError> {
        // Compressed:
        // let slot = self.pumps.get_mut(pump).ok_or(StationError::NoSuchPump(pump))?;
        let Some(slot) = self.pumps.get_mut(pump) else {
            return Err(StationError::NoSuchPump(pump));
        };
        if slot.is_some() {
            return Err(StationError::PumpBusy(pump));
        }
        // Compressed:
        // let cents_per_liter = *self.prices.get(&grade).ok_or(StationError::NoPrice(grade))?;
        let Some(price) = self.prices.get(&grade) else {
            return Err(StationError::NoPrice(grade));
        };
        let cents_per_liter = *price;

        // Compressed:
        // *slot = Some(Sale { grade, cents_per_liter, prepaid_cents, liters: 0 });
        let sale = Sale {
            grade,
            cents_per_liter,
            prepaid_cents,
            liters: 0,
        };
        *slot = Some(sale);
        Ok(())
    }

    /// Description: Dispenses fuel on an authorized pump. The pump stops early when the
    /// prepayment runs out or the tank runs dry.
    /// Inputs:
    /// - `pump`: the pump number.
    /// - `liters`: the liters the customer asks for.
    /// Outputs: the liters actually dispensed, or `NoSuchPump` or `NotAuthorized`.
    /// Constraints: the prepayment buys whole liters only.
    pub fn pump(&mut self, pump: usize, liters: u32) -> Result<u32, StationError> {
        // Compressed:
        // let sale = self.pumps.get_mut(pump).ok_or(StationError::NoSuchPump(pump))?
        //     .as_mut().ok_or(StationError::NotAuthorized(pump))?;
        let Some(slot) = self.pumps.get_mut(pump) else {
            return Err(StationError::NoSuchPump(pump));
        };
        let Some(sale) = slot else {
            return Err(StationError::NotAuthorized(pump));
        };

        // Compressed:
        // let tank = self.tanks.entry(sale.grade).or_insert(0);
        // let affordable = sale.prepaid_cents / sale.cents_per_liter - sale.liters;
        // let dispensed = liters.min(affordable).min(*tank);
        let tank            = self.tanks.entry(sale.grade).or_insert(0);
        let liters_paid_for = sale.prepaid_cents / sale.cents_per_liter;
        let affordable      = liters_paid_for - sale.liters;
        let can_buy         = min(liters, affordable);
        let dispensed       = min(can_buy, *tank);

        sale.liters += dispensed;
        *tank       -= dispensed;
        Ok(dispensed)
    }

    /// Description: Closes the sale on a pump, books the revenue and frees the pump.
    /// Inputs:
    /// - `pump`: the pump number.
    /// Outputs: the receipt for the sale, or `NoSuchPump` or `NotAuthorized`.
    pub fn finish(&mut self, pump: usize) -> Result<Receipt, StationError> {
        // Compressed:
        // let slot = self.pumps.get_mut(pump).ok_or(StationError::NoSuchPump(pump))?;
        // let sale = slot.take().ok_or(StationError::NotAuthorized(pump))?;
        let Some(slot) = self.pumps.get_mut(pump) else {
            return Err(StationError::NoSuchPump(pump));
        };
        let Some(sale) = slot.take() else {
            return Err(StationError::NotAuthorized(pump));
        };

        let charged_cents   = sale.liters * sale.cents_per_liter;
        self.revenue_cents += u64::from(charged_cents);

        // Compressed:
        // Ok(Receipt { grade: sale.grade, liters: sale.liters, charged_cents,
        //     refund_cents: sale.prepaid_cents - charged_cents })
        let refund_cents = sale.prepaid_cents - charged_cents;
        let receipt      = Receipt {
            grade:  sale.grade,
            liters: sale.liters,
            charged_cents,
            refund_cents,
        };
        Ok(receipt)
    }

    /// Description: Returns the fuel left of a grade.
    /// Inputs:
    /// - `grade`: the fuel grade.
    /// Outputs: the liters in that grade's tank, or 0 for a tank that was never filled.
    pub fn tank_level(&self, grade: Grade) -> u32 {
        // Compressed:
        // self.tanks.get(&grade).copied().unwrap_or(0)
        match self.tanks.get(&grade) {
            Some(liters) => *liters,
            None => 0,
        }
    }

    /// Description: Returns the money taken from finished sales.
    /// Inputs: none
    /// Outputs: the revenue in cents.
    pub fn revenue_cents(&self) -> u64 {
        self.revenue_cents
    }
}
