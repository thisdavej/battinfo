use battery::units::time::second;
use battery::units::*;
use battinfo::args::Args;
use battinfo::{
    BatteryInfo, TemperatureUnit, TimeFormat, format_temperature, format_time, get_fields,
};
use std::str::FromStr;

#[test]
fn test_temperature_unit_from_str() {
    assert_eq!(
        TemperatureUnit::from_str("celsius").unwrap(),
        TemperatureUnit::Celsius
    );
    assert_eq!(
        TemperatureUnit::from_str("C").unwrap(),
        TemperatureUnit::Celsius
    );
    assert_eq!(
        TemperatureUnit::from_str("fahrenheit").unwrap(),
        TemperatureUnit::Fahrenheit
    );
    assert_eq!(
        TemperatureUnit::from_str("F").unwrap(),
        TemperatureUnit::Fahrenheit
    );
    assert!(TemperatureUnit::from_str("invalid").is_err());
}

#[test]
fn test_time_format_from_str() {
    assert_eq!(TimeFormat::from_str("human").unwrap(), TimeFormat::Human);
    assert_eq!(
        TimeFormat::from_str("minutes").unwrap(),
        TimeFormat::Minutes
    );
    assert_eq!(TimeFormat::from_str("minute").unwrap(), TimeFormat::Minutes);
    assert_eq!(TimeFormat::from_str("min").unwrap(), TimeFormat::Minutes);
    assert!(TimeFormat::from_str("invalid").is_err());
}

#[test]
fn test_format_time_human() {
    let time = Some(Time::new::<second>(120.0));
    assert_eq!(format_time(time, TimeFormat::Human), "2m".to_string());
}

#[test]
fn test_format_time_minutes() {
    let time = Some(Time::new::<second>(120.0));
    assert_eq!(
        format_time(time, TimeFormat::Minutes),
        "2.0 minutes".to_string()
    );
}

#[test]
fn test_format_time_none() {
    assert_eq!(format_time(None, TimeFormat::Human), "N/A".to_string());
}

#[test]
fn test_format_temperature_none() {
    assert_eq!(
        format_temperature(None, TemperatureUnit::Celsius),
        "Unknown".to_string()
    );
}

#[test]
fn test_get_fields() {
    let battery_info = BatteryInfo {
        vendor: "TestVendor".to_string(),
        model: "TestModel".to_string(),
        serial_number: "TestSerial".to_string(),
        technology: "LithiumIon".to_string(),
        state: battery::State::Full,
        capacity: 95.0,
        temperature: "26.85 °C".to_string(),
        cycle_count: "100".to_string(),
        energy: 50.0,
        energy_full: 60.0,
        energy_full_design: 65.0,
        energy_rate: 10.0,
        voltage: 12.0,
        time_to_empty: "1 hour".to_string(),
        time_to_full: "30 minutes".to_string(),
        percent_full: 80.0,
    };

    let args = Args {
        vendor: true,
        model: true,
        serial_number: true,
        technology: true,
        capacity: true,
        percent_full: true,
        state: true,
        time_to_empty: true,
        time_to_full: true,
        temperature: true,
        cycle_count: false,
        energy: true,
        energy_full: true,
        energy_full_design: true,
        energy_rate: true,
        voltage: true,
        all: false,
        json: false,
        compact: false,
        battery_number: None,
        temperature_unit: TemperatureUnit::Celsius,
        time_format: TimeFormat::Human,
    };

    let fields = get_fields(&battery_info, &args);
    assert_eq!(fields.len(), 16);
}
