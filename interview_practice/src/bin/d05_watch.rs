//! Simulates a digital watch with a clock, an alarm and a stopwatch.
//!
//! APIs:
//! - `SECONDS_PER_DAY`: the length of one day in seconds
//! - `HourFormat`: the 24-hour or 12-hour face
//! - `WatchError`: why a time was rejected
//! - `Watch`: the clock, the alarm and the stopwatch
//! - `Watch::new`: a watch reading midnight
//! - `Watch::set_time`: set the clock
//! - `Watch::time`: the clock as hours, minutes and seconds
//! - `Watch::set_format`: choose the face
//! - `Watch::set_alarm`: set the alarm
//! - `Watch::clear_alarm`: turn the alarm off
//! - `Watch::tick`: advance one second
//! - `Watch::start_stopwatch`: start counting
//! - `Watch::stop_stopwatch`: pause counting
//! - `Watch::reset_stopwatch`: zero the stopwatch
//! - `Watch::stopwatch_seconds`: the stopwatch reading
//! - `Display for Watch`: the clock face as text

use std::fmt;

/// Description: The number of seconds in one day.
pub const SECONDS_PER_DAY: u32 = 24 * 60 * 60;

/// Description: How the clock face shows the hour.
/// Variants:
/// - `H24`: 00 to 23, zero-padded. This is the default.
/// - `H12`: 1 to 12 with an AM or PM suffix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HourFormat {
    #[default]
    H24,
    H12,
}

/// Description: The reason a time was rejected.
/// Variants:
/// - `InvalidTime`: the hour, minute or second is out of range.
#[derive(Debug, PartialEq)]
pub enum WatchError {
    InvalidTime { hour: u32, minute: u32, second: u32 },
}

/// Description: A digital watch.
/// Fields:
/// - `seconds`: the time of day as seconds since midnight.
/// - `format`: how the clock face shows the hour.
/// - `alarm`: the alarm time as seconds since midnight, or `None` when the alarm is off.
/// - `stopwatch`: the stopwatch reading in seconds.
/// - `running`: whether the stopwatch is counting.
#[derive(Debug, Default)]
pub struct Watch {
    seconds: u32,
    format: HourFormat,
    alarm: Option<u32>,
    stopwatch: u32,
    running: bool,
}

/// Description: Converts a time of day to seconds since midnight.
/// Inputs:
/// - `hour`: 0 to 23.
/// - `minute`: 0 to 59.
/// - `second`: 0 to 59.
/// Outputs: the seconds since midnight, or `InvalidTime` when any part is out of range.
fn to_seconds(hour: u32, minute: u32, second: u32) -> Result<u32, WatchError> {
    if hour < 24 && minute < 60 && second < 60 {
        Ok(hour * 3600 + minute * 60 + second)
    } else {
        Err(WatchError::InvalidTime { hour, minute, second })
    }
}

impl Watch {
    /// Description: Builds a watch reading midnight on the 24-hour face, with the alarm off and
    /// the stopwatch at zero.
    /// Inputs: none
    /// Outputs: the new watch.
    pub fn new() -> Self {
        Self::default()
    }

    /// Description: Sets the clock.
    /// Inputs:
    /// - `hour`: 0 to 23.
    /// - `minute`: 0 to 59.
    /// - `second`: 0 to 59.
    /// Outputs: `Ok(())` once set, or `InvalidTime`.
    /// Constraints: a rejected time leaves the clock unchanged.
    pub fn set_time(&mut self, hour: u32, minute: u32, second: u32) -> Result<(), WatchError> {
        self.seconds = to_seconds(hour, minute, second)?;
        Ok(())
    }

    /// Description: Returns the clock reading.
    /// Inputs: none
    /// Outputs: the hour, minute and second.
    pub fn time(&self) -> (u32, u32, u32) {
        (self.seconds / 3600, self.seconds / 60 % 60, self.seconds % 60)
    }

    /// Description: Chooses how the clock face shows the hour.
    /// Inputs:
    /// - `format`: the 24-hour or 12-hour face.
    /// Outputs: none
    pub fn set_format(&mut self, format: HourFormat) {
        self.format = format;
    }

    /// Description: Sets the alarm to ring at a time of day.
    /// Inputs:
    /// - `hour`: 0 to 23.
    /// - `minute`: 0 to 59.
    /// Outputs: `Ok(())` once set, or `InvalidTime`.
    pub fn set_alarm(&mut self, hour: u32, minute: u32) -> Result<(), WatchError> {
        self.alarm = Some(to_seconds(hour, minute, 0)?);
        Ok(())
    }

    /// Description: Turns the alarm off.
    /// Inputs: none
    /// Outputs: none
    pub fn clear_alarm(&mut self) {
        self.alarm = None;
    }

    /// Description: Advances the clock by one second, and advances the stopwatch when it is
    /// running.
    /// Inputs: none
    /// Outputs: `true` when this tick reaches the alarm time, otherwise `false`.
    /// Constraints: the clock wraps from 23:59:59 to 00:00:00.
    pub fn tick(&mut self) -> bool {
        self.seconds = (self.seconds + 1) % SECONDS_PER_DAY;
        if self.running {
            self.stopwatch += 1;
        }
        self.alarm == Some(self.seconds)
    }

    /// Description: Starts the stopwatch counting.
    /// Inputs: none
    /// Outputs: none
    pub fn start_stopwatch(&mut self) {
        self.running = true;
    }

    /// Description: Pauses the stopwatch and keeps its reading.
    /// Inputs: none
    /// Outputs: none
    pub fn stop_stopwatch(&mut self) {
        self.running = false;
    }

    /// Description: Sets the stopwatch reading back to zero.
    /// Inputs: none
    /// Outputs: none
    pub fn reset_stopwatch(&mut self) {
        self.stopwatch = 0;
    }

    /// Description: Returns the stopwatch reading.
    /// Inputs: none
    /// Outputs: the counted seconds.
    pub fn stopwatch_seconds(&self) -> u32 {
        self.stopwatch
    }
}

impl fmt::Display for Watch {
    /// Description: Writes the clock face, such as `13:05:09` or `1:05:09 PM`.
    /// Inputs:
    /// - `f`: the formatter to write into.
    /// Outputs: the formatter's result.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (hour, minute, second) = self.time();
        match self.format {
            HourFormat::H24 => write!(f, "{hour:02}:{minute:02}:{second:02}"),
            HourFormat::H12 => {
                let suffix = if hour < 12 { "AM" } else { "PM" };
                let hour = match hour % 12 {
                    0 => 12,
                    h => h,
                };
                write!(f, "{hour}:{minute:02}:{second:02} {suffix}")
            }
        }
    }
}

/// Description: Ticks past a 7:00 alarm, printing each second, then shows the 12-hour face.
/// Inputs: none
/// Outputs: none
fn main() {
    let mut watch = Watch::new();
    watch.set_time(6, 59, 58).unwrap();
    watch.set_alarm(7, 0).unwrap();
    for _ in 0..3 {
        let ringing = watch.tick();
        println!("{watch} ringing={ringing}");
    }
    watch.set_format(HourFormat::H12);
    println!("{watch}");
}
