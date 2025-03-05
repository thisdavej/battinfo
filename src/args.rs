use argh::FromArgs;
use crate::internal::{TemperatureUnit, TimeFormat};

#[derive(FromArgs)]
/// Fetch battery information
pub struct Args {
    /// display vendor
    #[argh(switch)]
    pub vendor: bool,

    /// display model
    #[argh(switch, short = 'm')]
    pub model: bool,

    /// display serial number
    #[argh(switch)]
    pub serial_number: bool,

    /// display technology
    #[argh(switch, short = 'g')]
    pub technology: bool,

    /// display capacity
    #[argh(switch)]
    pub capacity: bool,

    /// specify battery number to display (first battery is 1)
    #[argh(option, short = 'b')]
    pub battery_number: Option<usize>,

    /// display all battery field values
    #[argh(switch, short = 'a')]
    pub all: bool,

    /// print in compact mode
    #[argh(switch, short = 'c')]
    pub compact: bool,

    /// display percent full of charge
    #[argh(switch, short = 'p')]
    pub percent_full: bool,

    /// display battery state (Charging, Discharging, or Full)
    #[argh(switch, short = 's')]
    pub state: bool,

    /// display time to empty
    #[argh(switch, short = 'e')]
    pub time_to_empty: bool,

    /// display time to full
    #[argh(switch, short = 'f')]
    pub time_to_full: bool,

    /// specifies the time format for display. Options are 'human' for human-readable time and 'minutes' for time in minutes. Default is 'human'.
    #[argh(option, short = 'i', default = "TimeFormat::Human")]
    pub time_format: TimeFormat,

    /// display temperature
    #[argh(switch, short = 't')]
    pub temperature: bool,

    /// temperature unit (c for Celsius, f for Fahrenheit)
    #[argh(option, short = 'u', default = "TemperatureUnit::Fahrenheit")]
    pub temperature_unit: TemperatureUnit,

    /// display cycle count
    #[argh(switch, short = 'y')]
    pub cycle_count: bool,

    /// display energy
    #[argh(switch)]
    pub energy: bool,

    /// display energy full
    #[argh(switch, short = 'n')]
    pub energy_full: bool,

    /// display energy full design
    #[argh(switch, short = 'd')]
    pub energy_full_design: bool,

    /// display energy rate
    #[argh(switch, short = 'r')]
    pub energy_rate: bool,

    /// display voltage
    #[argh(switch, short = 'v')]
    pub voltage: bool,

    /// output in JSON format
    #[argh(switch, short = 'j')]
    pub json: bool,
}
