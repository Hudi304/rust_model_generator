use colored::Colorize;

pub fn print_delete_file_error(error: std::io::Error, file_path: &String) {
    println!(
        "{} {} {}",
        "ERROR on file DELETE".red(),
        &file_path,
        error.to_string()
    );
}

pub fn print_delete_file_success(file_path: &String) {
    println!("{} {}", "Deleted File :".yellow(), &file_path);
}

pub fn print_create_file_error(error: std::io::Error, file_path: &String) {
    let message = "Error During File Creation :".yellow();
    println!("{message} {file_path} \n : {}", error.to_string());
}

pub fn print_write_file_error(error: std::io::Error, file_path: &String) {
    let message = "Error During File Creation :".yellow();
    println!("{message} {file_path} \n : {}", error.to_string());
}

pub fn print_read_file_error(file_path: &String, error: std::io::Error) {
    let message = "Could not Read File".red();
    let file_path = file_path.white();
    println!("{message} {file_path} \n : {error}");
}

pub fn print_parsing_error(file_path: &String, error: toml::de::Error) {
    let message = "Could parse file contents of".red();
    let file_path = file_path.white();
    println!("{message} {file_path} \n : {error}");
}

pub fn print_write_file_success(file_path: &String) {
    let message = "Created File".cyan();
    let file_path = file_path.green();
    println!("{message} {file_path}");
}
