use std::fs;

use colored::Colorize;

#[derive(serde::Deserialize)]
pub struct ReplaceAllSettings {
    // filters: Filters,
    // replace: Replace,
}

#[derive(serde::Deserialize)]

pub struct GeneratorConfig {}

pub fn read_generator_config() -> GeneratorConfig {
    let settings_path = "src/tools/replace_all_config.toml";

    let file: String = match fs::read_to_string(settings_path) {
        Err(why) => {
            panic!("{}: {}", "couldn't open \n".red(), why);
        }
        Ok(file) => file,
    };

    let file_clone = file.clone();

    let config: GeneratorConfig = match toml::from_str(&file_clone) {
        Ok(d) => d,
        Err(_) => {
            panic!("Unable to load data from ");
        }
    };

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
