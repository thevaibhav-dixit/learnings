use std::path::Path;

pub fn take_input(file_name: &str) -> Result<String, std::io::Error> {
    let curr_dir = std::env::var("CURR_DIR").expect("ROOT environment variable not set");
    let path = Path::new(&curr_dir).join(file_name);

    Ok(std::fs::read_to_string(path)?)
}
