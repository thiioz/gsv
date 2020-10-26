use std::result::Result;

pub fn process() -> Result<(), String> {
    println!("gsv {}", env!("CARGO_PKG_VERSION"));

    return Ok(());
}