const MAX_BRIGHTNESS_FILEPATH: &str = "/sys/class/backlight/amdgpu_bl0/max_brightness";
const BRIGHTNESS_FILEPATH: &str = "/sys/class/backlight/amdgpu_bl0/brightness";
const STEP: u8 = 17;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = args[1].as_str();

    let brightness_val = read_int_from_file(BRIGHTNESS_FILEPATH);
    let max_brightness_val = read_int_from_file(MAX_BRIGHTNESS_FILEPATH);

    match command {
        "set" => {
            let new_value: u8 = args[2].parse().expect("Could not parse string to u8");

            std::fs::write(BRIGHTNESS_FILEPATH, new_value.to_string())
                .expect("Error setting new brightness");
        }

        "inc" => {
            let new_value = brightness_val.saturating_add(STEP);

            std::fs::write(BRIGHTNESS_FILEPATH, new_value.to_string())
                .expect("Error increasing brightness");

            show_notification(get_brightness_level(new_value, max_brightness_val).as_str());
        }

        "dec" => {
            let new_value = brightness_val.saturating_sub(STEP);

            std::fs::write(BRIGHTNESS_FILEPATH, new_value.to_string())
                .expect("Error decreasing brightness");

            show_notification(get_brightness_level(new_value, max_brightness_val).as_str());
        }

        "show" => {
            let bars = get_brightness_level(brightness_val, max_brightness_val);
            println!("{:?}", bars);
        }

        _ => println!("Invalid command: {:?}", command),
    }
}

fn read_int_from_file(path: &str) -> u8 {
    let mut content = std::fs::read_to_string(path).expect("Error opening a file");

    if content.ends_with('\n') {
        content.pop();
    }

    content.parse().expect("Error parsing string to u8")
}

fn get_brightness_level(brightness: u8, max_brightness: u8) -> String {
    let length = max_brightness / STEP;
    let filled_length = brightness / STEP;
    let mut bars = format!("{}/{} ", filled_length, length);

    for v in 1..length {
        if v <= filled_length {
            bars.push('█');
            continue;
        }

        bars.push('_');
    }

    bars
}

fn show_notification(brightness_level: &str) {
    std::process::Command::new("dunstify")
        .args(&["Brightness", brightness_level, "-r", "110001", "-t", "2000"])
        .spawn()
        .expect("Error executing shell command");
}
