use super::printers::{print_parsing_error, print_read_file_error};
use std::{fs, process};

#[derive(serde::Deserialize)]
pub struct ReplaceAllSettings {
    // filters: Filters,
    // replace: Replace,
}

#[derive(serde::Deserialize)]

pub struct GeneratorConfig {}

pub fn read_generator_config(generator_path: &String) -> GeneratorConfig {
    let file = fs::read_to_string(generator_path);

    let file = file.unwrap_or_else(|err| {
        print_read_file_error(generator_path, err);
        process::exit(1)
    });

    let config = toml::from_str::<GeneratorConfig>(&file);

    let config = config.unwrap_or_else(|err| {
        print_parsing_error(generator_path, err);
        process::exit(1)
    });

    return config;
}

// pub fn read_toml<'a, T: serde::Deserialize<'a>>() -> impl serde::Deserialize<'a> {
//     let settings_path = "src/tools/replace_all_config.toml";
//     let file: String = match fs::read_to_string(settings_path) {
//         Err(why) => {
//             panic!("{}: {}", "couldn't open \n".red(), why);
//         }
//         Ok(file) => file,
//     };
//     let file_clone = file.clone();
//     // how do you fix this?
//     let config: T = match toml::from_str(&file_clone) {
//         Ok(d) => d,
//         Err(_) => {
//             panic!("Unable to load data from ");
//         }
//     };
//     return config;
// }
