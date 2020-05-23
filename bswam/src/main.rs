use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("{}", reqwest::blocking::get("https://ifconfig.me")?.text()?);
    Ok(())
}
