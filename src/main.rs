use dirs::config_dir;
use serde::{Serialize, Deserialize};
use std::{
    fs::File,
    io::{Read, Write},
};
use xrandr::XHandle;

const FONT_SIZE_1_MONITOR: u8 = 8;
const FONT_SIZE_2_MONITOR: u8 = 12;
const ALACRITTY_CONFIG_PATH: &str = "alacritty/alacritty.yml";

fn main() {
    let mut alacritty_config = get_alacritty_config();
    let num_monitors = determine_number_of_physical_monitors();
    match num_monitors {
        0 => panic!("It should be impossible to have that many monitors."),
        1 => set_alacritty_font_size_1_monitor(&mut alacritty_config),
        2 => set_alacritty_font_size_2_monitor(&mut alacritty_config),
        _ => panic!("I don't have that many monitors."),
    };

    write_new_alacritty_config(&alacritty_config);
}

fn write_new_alacritty_config(alacritty_config: &MyAlacrittyConfig) {
    let yaml_string = serde_yaml::to_string(alacritty_config)
        .expect("Could not serialize MyAlacrittyConfig to string");

    let alacritty_config_path = config_dir()
        .expect("Cannot find config direcotry")
        .join(ALACRITTY_CONFIG_PATH);
    let mut new_config_file: File =
        File::create(alacritty_config_path).expect("Unable to create file");

    new_config_file
        .write_all(yaml_string.as_bytes())
        .expect("Could not wirte to alacritty config file");
}

fn get_alacritty_config() -> MyAlacrittyConfig {
    let alacritty_config_path = config_dir()
        .expect("Cannot find config direcotry")
        .join(ALACRITTY_CONFIG_PATH);
    let mut alacritty_config_file =
        File::open(alacritty_config_path).expect("Unable to open alacritty config");
    let mut contents = String::new();
    alacritty_config_file
        .read_to_string(&mut contents)
        .expect("unable to read file.");

    serde_yaml::from_str(&contents).expect("Could not deserialize alacritty_config.")
}

fn set_alacritty_font_size_1_monitor(alacritty_config: &mut MyAlacrittyConfig) {
    alacritty_config.font.size = FONT_SIZE_1_MONITOR;
}

fn set_alacritty_font_size_2_monitor(alacritty_config: &mut MyAlacrittyConfig) {
    alacritty_config.font.size = FONT_SIZE_2_MONITOR;
}

fn determine_number_of_physical_monitors() -> u8 {
    let mut xhandle = XHandle::open().unwrap();
    xhandle.monitors().unwrap().len().try_into().unwrap()
}

/// Yeah yeah i know its not a full alacritty config, but idc
#[derive(Deserialize, Serialize, Debug)]
struct MyAlacrittyConfig {
    scrolling: Scrolling,
    font: Font,
    /// this is actually a vec of file paths
    import: Vec<String>,
    window: Window,
}

#[derive(Deserialize, Serialize, Debug)]
struct Scrolling {
    history: u8,
}

#[derive(Deserialize, Serialize, Debug)]
struct Font {
    size: u8,
    normal: NormalFont,
}

#[derive(Deserialize, Serialize, Debug)]
struct NormalFont {
    family: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Window {
    opacity: f32,
}

#[cfg(test)]
mod test {
    use super::*;
    use dirs::config_dir;

    #[test]
    fn test() {
        get_alacritty_config();
        assert!(false)
    }
}
