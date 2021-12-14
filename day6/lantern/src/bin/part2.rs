use std::error::Error;
use std::fs::File;
use std::io::{Read, BufReader, BufRead};
use std::path::Path;
use std::cell::Cell;

struct Pop2 {
    fish: [u64; 9],
}

impl Pop2 {
    pub fn new() -> Pop2 {
        Pop2 { fish: [0; 9], }
    }

    pub fn advance(&mut self) {
        let fish0 = self.fish[0];
        let fish1 = self.fish[1];
        let fish2 = self.fish[2];
        let fish3 = self.fish[3];
        let fish4 = self.fish[4];
        let fish5 = self.fish[5];
        let fish6 = self.fish[6];
        let fish7 = self.fish[7];
        let fish8 = self.fish[8];
        self.fish[0] = fish1;
        self.fish[1] = fish2;
        self.fish[2] = fish3;
        self.fish[3] = fish4;
        self.fish[4] = fish5;
        self.fish[5] = fish6;
        self.fish[6] = fish7 + fish0;
        self.fish[7] = fish8;
        self.fish[8] = fish0;

    }
}

fn main() -> Result<(), Box<dyn Error + 'static + Send + Sync>> {

    let mut contents = String::new();
    let mut f = File::open("input.txt")?;
    f.read_to_string(&mut contents)?;

    let mut fishpop: Vec<usize> = contents.trim_end().split(',')
        .map(|num| num.parse().unwrap() ).collect();

    let mut pop = [0u64; 9];

    for f in fishpop {
        pop[f] += 1;
    }

    let mut pop = Pop2 { fish: pop };


    for i in 0..256 {
        println!("{}", i);
        pop.advance();
    }

    println!("{}", pop.fish.iter().sum::<u64>());

    Ok(())

}
