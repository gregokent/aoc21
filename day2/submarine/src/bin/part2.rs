use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Direction {
    Forward(isize),
    Up(isize),
    Down(isize),
}

impl TryFrom<&str> for Direction {
    type Error = &'static str;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        let mut split = val.split_ascii_whitespace();
        if let Some(direction) = split.next() {
            if let Some(value) = split.next() {
                let value = value.parse::<isize>().map_err(|_| "error parsing number")?;
                match direction {
                    "forward" => { return Ok(Direction::Forward(value)); },
                    "up" => { return Ok(Direction::Up(value)); },
                    "down" => { return Ok(Direction::Down(value)); },
                    _ => { return Err("invalid direction");}
                }
            }
        }
        Err("failed to convert")
    }
}

struct Submarine {
    depth: isize,
    position: isize,
    aim: isize,
}

impl Submarine {
    pub fn new() -> Submarine
    {
        Submarine {
            depth: 0,
            position: 0,
            aim: 0,
        }
    }

    pub fn apply_direction(&mut self, dir: Direction) {
        match dir {
            Direction::Forward(pos) => { 
                self.position += pos;
                self.depth += pos * self.aim;
            },
            Direction::Up(asc) => { self.aim -= asc;},
            Direction::Down(desc) => { self.aim += desc;},
        }
        println!("depth: {} position: {}", self.depth, self.position);
    }

    pub fn combine(&self) -> isize {
        self.position * self.depth
    }
}

fn main() -> Result<(), Box<dyn Error + 'static + Send + Sync>> {

    let file = File::open("input.txt")?;
    let bufr = io::BufReader::new(file);
    let lines = bufr.lines();

    let mut sub = Submarine::new();
    for line in lines {
        if let Ok(command) = line {
            let dir = Direction::try_from(command.as_str())?;
            sub.apply_direction(dir);
        }
    }

    println!("combined value: {}", sub.combine());
    Ok(())

}
