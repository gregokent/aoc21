use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const NUM_BITS: usize = 12;

fn main() -> Result<(), Box<dyn Error + 'static + Send + Sync>> {

    let file = File::open("input.txt")?;
    let bufr = io::BufReader::new(file);
    let lines = bufr.lines();

    let mut bincounter = [0;32];
    let mut all_vals: Vec<usize> = lines.into_iter()
        .filter(|line| line.is_ok())
        .filter_map(|line| {
            usize::from_str_radix(&line.unwrap(), 2).ok()
        }).collect();

    let mut shift_bits = NUM_BITS - 1;
    let mut working_vec: Vec<usize> = all_vals.clone();
    let mut ones = Vec::new();
    let mut zeroes = Vec::new();
    while working_vec.len() != 1 {
        for val in working_vec {
            if (val & (1 << shift_bits)) == 0 {
                zeroes.push(val);
            } else {
                ones.push(val);
            }
        }
        println!("{} {}", ones.len(), zeroes.len());
        if ones.len() < zeroes.len() {
            working_vec = zeroes.clone();
        } else {
            working_vec = ones.clone();
        }
        shift_bits = shift_bits.saturating_sub(1);
        zeroes.clear();
        ones.clear();
    }
    let oxygen = working_vec[0];
    println!("{:b}", working_vec[0]);

    let mut shift_bits = NUM_BITS - 1;
    println!("{}", all_vals.len());
    working_vec = all_vals;
    while working_vec.len() != 1 {
    println!("{}", working_vec.len());
        for val in working_vec {
            if (val & (1 << shift_bits)) == 0 {
                zeroes.push(val);
            } else {
                ones.push(val);
            }
        }
        println!("{} {}", zeroes.len(), ones.len());
        if ones.len() < zeroes.len() {
            working_vec = ones.clone();
        } else {
            working_vec = zeroes.clone();
        }
        shift_bits = shift_bits.saturating_sub(1);
        zeroes.clear();
        ones.clear();
    }
    let co2 = working_vec[0];
    println!("{:012b}", working_vec[0]);
    println!("{}", oxygen * co2);
    Ok(())

}
