const MAX_BRIGHTNESS_FILEPATH: &str = "/sys/class/backlight/amdgpu_bl0/max_brightness";
const BRIGHTNESS_FILEPATH: &str = "/sys/class/backlight/amdgpu_bl0/brightness";
const STEP: u16 = 17;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = args[1].as_str();

    let brightness_val = read_int_from_file(BRIGHTNESS_FILEPATH);
    let max_brightness_val = read_int_from_file(MAX_BRIGHTNESS_FILEPATH);

    match command {
        "set" => {
            let mut new_value = args[2].parse().expect("Could not parse string to u16");

            if new_value > max_brightness_val {
                new_value = max_brightness_val;
            }

            std::fs::write(BRIGHTNESS_FILEPATH, new_value.to_string())
                .expect("Error setting new brightness");
        }

        "inc" => {
            let mut new_value = brightness_val + STEP;

            if new_value > max_brightness_val {
                new_value = max_brightness_val;
            }

            std::fs::write(BRIGHTNESS_FILEPATH, new_value.to_string())
                .expect("Error increasing brightness");
        }

        "dec" => {
            let new_value = brightness_val.saturating_sub(STEP);

            std::fs::write(BRIGHTNESS_FILEPATH, new_value.to_string())
                .expect("Error decreasing brightness");
        }

        "show" => {
            let length = max_brightness_val / STEP;
            let filled_length = brightness_val / STEP;
            let mut bars = String::from("");

            for v in 1..length {
                if v <= filled_length {
                    bars.push('█');
                    continue;
                }

                bars.push('_');
            }

            println!("{}/{} {:?}", filled_length, length, bars);
        }

        _ => println!("Invalid command: {:?}", command),
    }
}

fn read_int_from_file(path: &str) -> u16 {
    let mut content = std::fs::read_to_string(path).expect("Error opening a file");

    if content.ends_with("\n") {
        content.pop();
    }

    content.parse().expect("Error parsing string to u16")
}
