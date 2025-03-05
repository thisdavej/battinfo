use battinfo::args::Args;
use battinfo::{get_all_batteries_info, print_battery_info, print_compact_battery_info, print_battery_info_json};

fn main() {
    let args: Args = argh::from_env();

    match get_all_batteries_info(args.temperature_unit, args.time_format) {
        Ok(batteries_info) => {
            let battery_count = batteries_info.len();
            if let Some(battery_number) = args.battery_number {
                if battery_number == 0 || battery_number > battery_count {
                    eprintln!(
                        "Invalid battery number. Number of batteries available = {}",
                        battery_count
                    );
                    return;
                }
                let battery_info = &batteries_info[battery_number - 1];
                if args.json {
                    print_battery_info_json(battery_info, &args);
                } else if args.compact {
                    print_compact_battery_info(battery_number - 1, battery_info, battery_count);
                } else {
                    print_battery_info(battery_number - 1, battery_info, &args, battery_count);
                }
            } else if args.json {
                for battery_info in &batteries_info {
                    print_battery_info_json(battery_info, &args);
                }
            } else {
                for (index, battery_info) in batteries_info.iter().enumerate() {
                    if args.compact {
                        print_compact_battery_info(index, battery_info, battery_count);
                    } else {
                        print_battery_info(index, battery_info, &args, battery_count);
                    }
                }
            }
        }
        Err(e) => eprintln!("Failed to get battery info: {:?}", e),
    }
}
