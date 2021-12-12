use std::error::Error;
use std::fs::File;
use std::io::{Read, BufReader, BufRead};
use std::path::Path;
use std::cell::Cell;

struct Population {
    fish: Vec<LanternFish>,
}

impl Population {
    pub fn new() -> Population {
        Population {
            fish: Vec::new(),
        }
    }

    pub fn advance(&mut self) {
        let add_new = self.fish.iter().filter(|f| f.0.get() == 0).count();
        for fishie in &self.fish {
            fishie.advance();
        }

        for _ in 0..add_new {
            self.fish.push(LanternFish::from(8));
        }

    }
}

    
struct LanternFish(Cell<u8>);
impl From<u8> for LanternFish {
    fn from(val: u8) -> LanternFish {
        LanternFish(Cell::new(val))
    }
}
impl LanternFish {
    fn advance(&self) {
        match self.0.get() {
            0 => { self.0.set(6); },
            x @ 1..=8 => { self.0.set(x-1) },
            _ => unreachable!()
        }
    }
}


fn main() -> Result<(), Box<dyn Error + 'static + Send + Sync>> {

    let mut contents = String::new();
    let mut f = File::open("input.txt")?;
    f.read_to_string(&mut contents)?;

    let fishpop: Vec<LanternFish> = contents.trim_end().split(',')
        .map(|num| { LanternFish::from(num.parse::<u8>().unwrap()) }).collect();
    let mut pop = Population { fish: fishpop };

    for _ in 0..80 {
        pop.advance();
    }

    println!("{}", pop.fish.len());

    Ok(())

}
