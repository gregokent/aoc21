use std::error::Error;
use std::fs::File;
use std::io::{Read, BufReader, BufRead};
use std::path::Path;
use std::cell::Cell;

struct Population {
    fish: Vec<LanternFish>,
}

struct Pop2 {
    fish: [u64; 9];
}

impl Pop2 {
    pub fn new() -> Pop2 {
        Pop2 { fish: [0; 9], }
    }

    pub fn advance(&mut self) {

    }
}

impl Population {
    pub fn new() -> Population {
        Population {
            fish: Vec::new(),
        }
    }

    pub fn advance(&mut self) {
        //let add_new = self.fish.iter().filter(|f| f.0.get() == 0).count();
        let mut add_new: u64 = 0;
        for mut fishie in &mut self.fish {
            //fishie.advance();
            if fishie.advance() { add_new += 1; }
        }

        for _ in 0..add_new {
            //self.fish.push(LanternFish::from(8));
            self.fish.push(LanternFish(8));
        }

    }
}

    
struct LanternFish(u8);
impl From<u8> for LanternFish {
    fn from(val: u8) -> LanternFish {
        LanternFish(val)
    }
}
impl LanternFish {
    fn advance(&mut self) -> bool {
        match self.0 {
            0 => { self.0 = 6; true},
            1 => { self.0 = 0; false },
            x @ 2..=8 => { self.0 = x-1; false },
            _ => unreachable!()
        }
    }
}


fn main() -> Result<(), Box<dyn Error + 'static + Send + Sync>> {

    let mut contents = String::new();
    let mut f = File::open("input.txt")?;
    f.read_to_string(&mut contents)?;

    let mut fishpop: Vec<LanternFish> = contents.trim_end().split(',')
        .map(|num| { LanternFish::from(num.parse::<u8>().unwrap()) }).collect();

    fishpop.reserve(u32::MAX as usize);
    let mut pop = Population { fish: fishpop };

    for i in 0..256 {
        println!("{}", i);
        pop.advance();
    }

    println!("{}", pop.fish.len());

    Ok(())

}
