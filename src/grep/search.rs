use super::config::Config;
use std::error::Error;
use std::fs;

/// Runs the grep command
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path())?;
    println!("{contents}");
    Ok(())
}
