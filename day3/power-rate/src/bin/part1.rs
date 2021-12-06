use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error + 'static + Send + Sync>> {

    let file = File::open("input.txt")?;
    let bufr = io::BufReader::new(file);
    let lines = bufr.lines();

    let mut bincounter = [0;32];
    for line in lines {
        if let Ok(value) = line {
            let val = usize::from_str_radix(&value, 2)?;
            for (idx, _) in &mut bincounter.into_iter().enumerate() {
                bincounter[idx] += (val >> idx) & 0x1;
            }
        }
    }

    let mut gamma = 0;
    for (idx, numbits) in bincounter.into_iter().enumerate() {
        gamma |= if numbits > 500 { 1 } else { 0 } << idx;
    }
    let epsilon = !gamma;
    let epsilon = epsilon & 0xFFF;
    println!("{:?}", bincounter);
    println!("gamma: {},   {:#014b}", gamma, gamma);
    println!("episoln: {}, {:#014b}", epsilon, epsilon);
    println!("power = {}", epsilon * gamma);
    Ok(())

}
