use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error + 'static + Send + Sync>> {

    let file = File::open("input.txt")?;
    let bufr = io::BufReader::new(file);
    let lines = bufr.lines();

    for line in lines {
    }
    Ok(())

}
