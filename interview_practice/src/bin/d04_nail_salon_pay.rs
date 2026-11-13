//! Checks out nail salon tickets and tracks what each technician has earned.
//!
//! APIs:
//! - `CARD_FEE_PERCENT`: the card processor's cut
//! - `Payment`: how the client pays
//! - `PayError`: why a ticket was refused
//! - `Receipt`: the money split for one ticket
//! - `Salon`: the menu, the staff and the day's balances
//! - `Salon::new`: an empty salon
//! - `Salon::add_service`: put a service on the menu
//! - `Salon::hire`: add a technician with a commission rate
//! - `Salon::checkout`: price a ticket and split the money
//! - `Salon::earnings`: what a technician has earned today
//! - `Salon::salon_net_cents`: what the salon has kept today
//! - `Salon::end_of_day`: pay everyone and reset the balances

use std::collections::HashMap;

/// Description: The percent of a card payment that the card processor keeps.
pub const CARD_FEE_PERCENT: u32 = 3;

/// Description: How the client pays.
/// Variants:
/// - `Cash`: paid in cash, with no fee.
/// - `Card`: paid by card, and the processor keeps `CARD_FEE_PERCENT` of the charge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Payment {
    Cash,
    Card,
}

/// Description: The reason a ticket was refused.
/// Variants:
/// - `EmptyTicket`: the ticket lists no services.
/// - `UnknownTechnician`: nobody with this name works here.
/// - `UnknownService`: the menu has no service with this name.
#[derive(Debug, PartialEq)]
pub enum PayError {
    EmptyTicket,
    UnknownTechnician(String),
    UnknownService(String),
}

/// Description: How the money from one ticket was split.
/// Fields:
/// - `subtotal_cents`: the menu price of the services.
/// - `tip_cents`: the tip, which goes to the technician in full.
/// - `charged_cents`: the subtotal plus the tip, which is what the client pays.
/// - `card_fee_cents`: the processor's cut, which is zero for cash.
/// - `technician_cents`: the technician's commission plus the tip.
/// - `salon_cents`: what the salon keeps after the commission and the card fee.
#[derive(Debug, PartialEq)]
pub struct Receipt {
    pub subtotal_cents: u32,
    pub tip_cents: u32,
    pub charged_cents: u32,
    pub card_fee_cents: u32,
    pub technician_cents: u32,
    pub salon_cents: i64,
}

/// Description: The salon's menu, staff and running balances for the day.
/// Fields:
/// - `menu`: the price in cents of each service.
/// - `commission`: each technician's commission percent.
/// - `earnings`: what each technician has earned today, in cents.
/// - `salon_net_cents`: what the salon has kept today, in cents.
/// Constraints: `salon_net_cents` is signed because a large card tip can make the card fee
/// bigger than the salon's share.
#[derive(Default)]
pub struct Salon {
    menu: HashMap<String, u32>,
    commission: HashMap<String, u32>,
    earnings: HashMap<String, u32>,
    salon_net_cents: i64,
}

impl Salon {
    /// Description: Builds a salon with an empty menu, no staff and zero balances.
    /// Inputs: none
    /// Outputs: the new salon.
    pub fn new() -> Self {
        Self::default()
    }

    /// Description: Puts a service on the menu, or changes its price.
    /// Inputs:
    /// - `name`: the service name, such as "gel manicure".
    /// - `price_cents`: the price.
    /// Outputs: none
    pub fn add_service(&mut self, name: &str, price_cents: u32) {
        self.menu.insert(name.to_string(), price_cents);
    }

    /// Description: Adds a technician, or changes their commission rate.
    /// Inputs:
    /// - `name`: the technician's name.
    /// - `commission_percent`: the percent of the service price the technician earns.
    /// Outputs: none
    /// Constraints: panics when `commission_percent` is above 100.
    pub fn hire(&mut self, name: &str, commission_percent: u32) {
        assert!(commission_percent <= 100, "a commission is a percent of the price");
        self.commission.insert(name.to_string(), commission_percent);
    }

    /// Description: Prices a ticket, splits the money and adds it to the day's balances.
    /// Inputs:
    /// - `technician`: who did the work.
    /// - `services`: the menu names of the services performed.
    /// - `tip_cents`: the tip.
    /// - `payment`: how the client pays.
    /// Outputs: the receipt, or `EmptyTicket`, `UnknownTechnician` or `UnknownService`.
    /// Constraints: the commission and the card fee round down to whole cents. A refused ticket
    /// changes no balance.
    pub fn checkout(
        &mut self,
        technician: &str,
        services: &[&str],
        tip_cents: u32,
        payment: Payment,
    ) -> Result<Receipt, PayError> {
        if services.is_empty() {
            return Err(PayError::EmptyTicket);
        }
        let percent = *self
            .commission
            .get(technician)
            .ok_or_else(|| PayError::UnknownTechnician(technician.to_string()))?;
        let subtotal_cents = services
            .iter()
            .map(|&name| self.price_of(name))
            .sum::<Result<u32, PayError>>()?;

        let charged_cents = subtotal_cents + tip_cents;
        let card_fee_cents = match payment {
            Payment::Cash => 0,
            Payment::Card => charged_cents * CARD_FEE_PERCENT / 100,
        };
        let commission_cents = subtotal_cents * percent / 100;
        let technician_cents = commission_cents + tip_cents;
        let salon_cents = i64::from(subtotal_cents)
            - i64::from(commission_cents)
            - i64::from(card_fee_cents);

        *self.earnings.entry(technician.to_string()).or_insert(0) += technician_cents;
        self.salon_net_cents += salon_cents;
        Ok(Receipt {
            subtotal_cents,
            tip_cents,
            charged_cents,
            card_fee_cents,
            technician_cents,
            salon_cents,
        })
    }

    /// Description: Looks up the menu price of one service.
    /// Inputs:
    /// - `name`: the service name.
    /// Outputs: the price in cents, or `UnknownService`.
    fn price_of(&self, name: &str) -> Result<u32, PayError> {
        self.menu.get(name).copied().ok_or_else(|| PayError::UnknownService(name.to_string()))
    }

    /// Description: Returns what a technician has earned today.
    /// Inputs:
    /// - `technician`: the technician's name.
    /// Outputs: the cents earned, or 0 for someone who has earned nothing yet.
    pub fn earnings(&self, technician: &str) -> u32 {
        self.earnings.get(technician).copied().unwrap_or(0)
    }

    /// Description: Returns what the salon has kept today.
    /// Inputs: none
    /// Outputs: the salon's net in cents.
    pub fn salon_net_cents(&self) -> i64 {
        self.salon_net_cents
    }

    /// Description: Pays out every technician who earned money today and starts a fresh day.
    /// Inputs: none
    /// Outputs: each technician's name and pay in cents, sorted by name.
    /// Constraints: afterwards every balance, including the salon's, is zero.
    pub fn end_of_day(&mut self) -> Vec<(String, u32)> {
        let mut payouts: Vec<(String, u32)> = self.earnings.drain().collect();
        payouts.sort();
        self.salon_net_cents = 0;
        payouts
    }
}

/// Description: Checks out one card ticket and prints the receipt and the day's payroll.
/// Inputs: none
/// Outputs: none
fn main() {
    let mut salon = Salon::new();
    salon.add_service("gel manicure", 3_500);
    salon.add_service("pedicure", 4_500);
    salon.hire("Mai", 60);
    let receipt = salon.checkout("Mai", &["gel manicure", "pedicure"], 1_000, Payment::Card);
    println!("{receipt:?}");
    println!("payroll: {:?}", salon.end_of_day());
}
