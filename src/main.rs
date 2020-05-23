use std::{error, process};

fn main() -> Result<(), Box<dyn error::Error>> {
    process::exit(env!("BSWAM_STATUS").parse()?);
}
