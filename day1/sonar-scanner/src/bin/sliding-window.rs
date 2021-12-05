
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
    let depths: Vec<isize> = lines.into_iter().map(|s| {
        s.unwrap().parse::<isize>().unwrap()
    }).collect();

    for window in depths.windows(3) {
        let sum = window.iter().sum();
        if previous_val < sum { increasing += 1; };
        previous_val = sum;
    }
    println!("number of increasing depths: {}", increasing - 1);

    Ok(())

}
