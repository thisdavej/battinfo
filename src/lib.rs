#[doc(hidden)]
pub mod args;

pub mod internal {
    use std::io;
    use battery::{Manager, Battery, Error};
    use std::time::Duration;
    use human_time::ToHumanTimeString;
    use serde_json::json;
    use crate::args::Args;

    #[derive(Debug)]
    pub struct BatteryInfo {
        pub vendor: String,
        pub model: String,
        pub serial_number: String,
        pub technology: String,
        pub state: battery::State,
        pub capacity: f32,
        pub temperature: String,
        pub cycle_count: String,
        pub energy: f32,
        pub energy_full: f32,
        pub energy_full_design: f32,
        pub energy_rate: f32,
        pub voltage: f32,
        pub time_to_empty: String,
        pub time_to_full: String,
        pub percent_full: f32,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum TemperatureUnit {
        Celsius,
        Fahrenheit,
    }

    impl std::str::FromStr for TemperatureUnit {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                "celsius" | "c" => Ok(TemperatureUnit::Celsius),
                "fahrenheit" | "f" => Ok(TemperatureUnit::Fahrenheit),
                _ => Err(format!("Invalid temperature unit: {}", s)),
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum TimeFormat {
        Human,
        Minutes,
    }

    impl std::str::FromStr for TimeFormat {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                "human" => Ok(TimeFormat::Human),
                "minutes" | "minute" | "min" => Ok(TimeFormat::Minutes),
                _ => Err(format!("Invalid time format: {}", s)),
            }
        }
    }

    pub fn get_all_batteries_info(temp_unit: TemperatureUnit, time_format: TimeFormat) -> Result<Vec<BatteryInfo>, Error> {
        let manager = Manager::new()?;
        let mut batteries_info = Vec::new();

        for maybe_battery in manager.batteries()? {
            match maybe_battery {
                Ok(battery) => batteries_info.push(convert_battery_info(&battery, temp_unit, time_format)),
                Err(e) => eprintln!("Error reading battery: {:?}", e),
            }
        }

        if batteries_info.is_empty() {
            eprintln!("No batteries found");
            return Err(io::Error::from(io::ErrorKind::NotFound).into());
        }

        Ok(batteries_info)
    }

    fn convert_battery_info(battery: &Battery, temp_unit: TemperatureUnit, time_format: TimeFormat) -> BatteryInfo {
        BatteryInfo {
            vendor: battery.vendor().unwrap_or("Unknown").to_string(),
            model: battery.model().unwrap_or("Unknown").to_string(),
            serial_number: battery.serial_number().unwrap_or("Unknown").trim().to_string(),
            technology: format!("{:?}", battery.technology()),
            state: battery.state(),
            capacity: battery.state_of_health().value * 100.0,
            temperature: format_temperature(battery.temperature(), temp_unit),
            cycle_count: battery.cycle_count().map_or("Unknown".to_string(), |count| count.to_string()),
            energy: battery.energy().value,
            energy_full: battery.energy_full().value,
            energy_full_design: battery.energy_full_design().value,
            energy_rate: battery.energy_rate().value,
            voltage: battery.voltage().value,
            time_to_empty: format_time(battery.time_to_empty(), time_format),
            time_to_full: format_time(battery.time_to_full(), time_format),
            percent_full: battery.state_of_charge().value * 100.0,
        }
    }

    pub fn format_time(time: Option<battery::units::Time>, time_format: TimeFormat) -> String {
        match time {
            Some(t) => match time_format {
                TimeFormat::Human => Duration::from_secs(t.value as u64).to_human_time_string(),
                TimeFormat::Minutes => format!("{:.1} minutes", t.value as f64 / 60.0),
            },
            None => "N/A".to_string(),
        }
    }

    pub fn format_temperature(temp: Option<battery::units::ThermodynamicTemperature>, unit: TemperatureUnit) -> String {
        match temp {
            Some(t) => match unit {
                TemperatureUnit::Celsius => format!("{:.2} °C", t.value),
                TemperatureUnit::Fahrenheit => format!("{:.2} °F", t.value * 9.0 / 5.0 + 32.0),
            },
            None => "Unknown".to_string(),
        }
    }

    pub fn get_fields<'a>(battery_info: &'a BatteryInfo, args: &'a Args) -> Vec<(&'a str, bool, String, &'a str)> {
        vec![
            (
                "Vendor",
                args.vendor || args.all,
                battery_info.vendor.to_string(),
                "vendor"
            ),
            (
                "Model",
                args.model || args.all,
                battery_info.model.to_string(),
                "model"
            ),
            (
                "Serial Number",
                args.serial_number || args.all,
                battery_info.serial_number.to_string(),
                "serial_number"
            ),
            (
                "Technology",
                args.technology || args.all,
                battery_info.technology.to_string(),
                "technology"
            ),
            (
                "Capacity",
                args.capacity || args.all,
                format!("{:.1}%", battery_info.capacity),
                "capacity"
            ),
            (
                "Percent Full",
                args.percent_full || args.all,
                format!("{:.1}%", battery_info.percent_full),
                "percent_full"
            ),
            (
                "State",
                args.state || args.all,
                format!("{:?}", battery_info.state),
                "state"
            ),
            (
                "Time to Empty",
                args.time_to_empty || args.all,
                battery_info.time_to_empty.to_string(),
                "time_to_empty"
            ),
            (
                "Time to Full",
                args.time_to_full || args.all,
                battery_info.time_to_full.to_string(),
                "time_to_full"
            ),
            (
                "Temperature",
                args.temperature || args.all,
                battery_info.temperature.to_string(),
                "temperature"
            ),
            (
                "Cycle Count",
                args.cycle_count || args.all,
                battery_info.cycle_count.to_string(),
                "cycle_count"
            ),
            (
                "Energy",
                args.energy || args.all,
                format!("{:.1} Wh", battery_info.energy),
                "energy"
            ),
            (
                "Energy Full",
                args.energy_full || args.all,
                format!("{:.1} Wh", battery_info.energy_full),
                "energy_full"
            ),
            (
                "Energy Full Design",
                args.energy_full_design || args.all,
                format!("{:.1} Wh", battery_info.energy_full_design),
                "energy_full_design"
            ),
            (
                "Energy Rate",
                args.energy_rate || args.all,
                format!("{:.1} W", battery_info.energy_rate),
                "energy_rate"
            ),
            (
                "Voltage",
                args.voltage || args.all,
                format!("{:.1} V", battery_info.voltage),
                "voltage"
            ),
        ]
    }

    pub fn print_battery_info(index: usize, battery_info: &BatteryInfo, args: &Args, battery_count: usize) {
        let fields = get_fields(battery_info, args);
        let any_selected = fields.iter().any(|(_, enabled, _, _)| *enabled);
        let max_length = fields
            .iter()
            .filter(|(_, enabled, _, _)| *enabled || !any_selected)
            .map(|(name, _, _, _)| name.len())
            .max()
            .unwrap_or(0);
        let pad = max_length;

        if battery_count > 1 {
            println!("Battery {}:", index + 1);
        }

        for (name, enabled, value, _) in &fields {
            if *enabled || !any_selected {
                println!("  {:<width$} : {}", name, value, width = pad);
            }
        }
    }

    pub fn print_compact_battery_info(index: usize, battery_info: &BatteryInfo, battery_count: usize) {
        let additional_info = match battery_info.state {
            battery::State::Discharging => {
                format!("(⇣ discharging - empty in {})", battery_info.time_to_empty)
            }
            battery::State::Charging => format!("(⇡ charging - full in {})", battery_info.time_to_full),
            _ => String::new(),
        };

        if battery_count > 1 {
            println!(
                "Battery {}: {:.1}% {}",
                index + 1,
                battery_info.percent_full,
                additional_info
            );
        } else {
            println!(
                "Battery: {:.1}% {}",
                battery_info.percent_full, additional_info
            );
        }
    }

    pub fn print_battery_info_json(battery_info: &BatteryInfo, args: &Args) {
        let fields = get_fields(battery_info, args);
        let mut json_map = serde_json::Map::new();
        let any_selected = fields.iter().any(|(_, enabled, _, _)| *enabled);

        for (_, enabled, value, json_field) in &fields {
            if *enabled || args.all || !any_selected {
                json_map.insert((*json_field).to_string(), json!(value));
            }
        }

        let json_output = serde_json::Value::Object(json_map);
        println!("{}", json_output);
    }
}

// Re-export hidden internal functions for use within the crate
#[doc(hidden)]
pub use internal::*;