# battinfo

A command-line tool for retrieving battery information, built using the excellent cross-platform `battery` crate.

## Installation

```bash
cargo install battinfo
```

## Usage

```bash
battinfo --help
Usage: battinfo [--vendor] [-m] [--serial-number] [-g] [--capacity] [-b <battery-number>]
                [-a] [-c] [-p] [-s] [-e] [-f] [-i <time-format>] [-t] [-u <temperature-unit>]
                [-y] [--energy] [-n] [-d] [-r] [-v] [-j]

Fetch battery information

Options:
  --vendor          display vendor
  -m, --model       display model
  --serial-number   display serial number
  -g, --technology  display technology
  --capacity        display capacity
  -b, --battery-number
                    specify battery number to display (first battery is 1)
  -a, --all         display all battery field values
  -c, --compact     print in compact mode
  -p, --percent-full
                    display percent full of charge
  -s, --state       display battery state (Charging, Discharging, or Full)
  -e, --time-to-empty
                    display time to empty
  -f, --time-to-full
                    display time to full
  -i, --time-format specifies the time format for display. Options are 'human'
                    for human-readable time and 'minutes' for time in minutes.
                    Default is 'human'.
  -t, --temperature display temperature
  -u, --temperature-unit
                    temperature unit (c for Celsius, f for Fahrenheit)
  -y, --cycle-count display cycle count
  --energy          display energy
  -n, --energy-full display energy full
  -d, --energy-full-design
                    display energy full design
  -r, --energy-rate display energy rate
  -v, --voltage     display voltage
  -j, --json        output in JSON format
  --help, help      display usage information
```

### Examples

- Display all battery info

    ```bash
    battinfo --all

    # This can also be accomplished by omitting all arguments
    battinfo
    ```

    Output:

    ```text
    Vendor             : LGC-LGC6.73
    Model              : DELL H754V79
    Serial Number      : 9001
    Technology         : LithiumPolymer
    Capacity           : 100.0%
    Percent Full       : 84.1%
    State              : Discharging
    Time to Empty      : 2h,30m,31s
    Time to Full       : N/A
    Temperature        : Unknown
    Cycle Count        : Unknown
    Energy             : 187197.1 Wh
    Energy Full        : 187197.1 Wh
    Energy Full Design : 187197.1 Wh
    Energy Rate        : 9.4 W
    Voltage            : 8.5 V
    ```

- Display basic battery information in compact format:

    ```bash
    # long argument option
    battinfo --compact

    # short argument option
    battinfo -c
    ```

    Output:

    ```text
    Battery: 70.4% (⇣ discharging - empty in 6h,40m,38s)
    ```

- Display detailed battery information on percent full, battery state, and time to empty:

    ```bash
    # short argument options
    battinfo -p -s -e

    # long argument options
    battinfo --percent-full --state --time-to-empty
    ```

    Output:

    ```text
    Percent Full  : 69.0%
    State         : Discharging
    Time to Empty : 3h,5m,7s
    ```

- Output battery information in JSON format:

    ```bash
    battinfo -p -s -v --json
    ```

    Output:

    ```json
    {"percent_full":"70.1%","state":"Discharging","voltage":"7.9 V"}
    ```

- Output all battery information in JSON format:

    ```bash
    battinfo --all --json
    ```

    Output:

    ```json
    {"capacity":"100.0%","cycle_count":"Unknown","energy":"186020.6 Wh","energy_full":"187197.1 Wh","energy_full_design":"187197.1 Wh","energy_rate":"7.4 W","model":"DELL H754V79","percent_full":"84.1%","serial_number":"9001","state":"Discharging","technology":"LithiumPolymer","temperature":"Unknown","time_to_empty":"2h,1m,25s","time_to_full":"N/A","vendor":"LGC-LGC6.73","voltage":"8.5 V"}
    ```

## Building from Source

If you want to build `battinfo` from source, you'll need to have Rust and Cargo installed.

1. Clone the repository.
2. Navigate to the project directory.
3. Run `cargo build`.

The executable will be located in the `target/debug` directory.
