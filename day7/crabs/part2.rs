use std::error::Error;
use std::fs::File;
use std::io::{Read, BufReader, BufRead};
use std::path::Path;
use std::cell::Cell;



fn main() -> Result<(), Box<dyn Error + 'static + Send + Sync>> {

    let mut contents = String::new();
    let mut f = File::open("input.txt")?;
    f.read_to_string(&mut contents)?;

    let crabs: Vec<i16> = contents.trim_end().split(',')
        .map(|num| num.parse::<i16>().unwrap()).collect();

    let max_pos = &crabs.iter().max().unwrap();
    println!("max pos {}", max_pos);

    let mut fuel_spent = Vec::new();
    for i in 0..**max_pos {
        let fuel = crabs.iter().fold(0, |acc, &crab| { let x = (i-crab).abs() as u64; let sum = (x*(x+1))/2;  acc + sum });
        fuel_spent.push(fuel);
    }

    println!("{}", fuel_spent.iter().min().unwrap());

    Ok(())

}
