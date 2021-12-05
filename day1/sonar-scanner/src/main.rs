
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error + 'static + Send + Sync>> {

    let file = File::open("input.txt")?;
    let bufr = io::BufReader::new(file);
    let lines = bufr.lines();

    let mut increasing = 0;

    let mut previous_val = 0;
    for line in lines {
        if let Ok(depth) = line {
            let depth = depth.parse::<isize>().unwrap();
            if previous_val < depth { increasing += 1; };
            previous_val = depth;
        }
    }
    println!("number of increasing depths: {}", increasing - 1);

    Ok(())

}
