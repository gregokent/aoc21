use std::error::Error;
use std::fs::File;
use std::io::{Read, BufRead};
use std::path::Path;

#[derive(Debug, Copy, Clone)]
enum Square {
    Marked(usize),
    Unmarked(usize),
}

impl From<usize> for Square {
    fn from(val: usize) -> Square {
        Square::Unmarked(val)
    }
}

impl Square {
    pub fn new() -> Square {
        Square::Unmarked(0)
    }

    pub fn mark(self) -> Square {
        match self {
            Square::Unmarked(x) => Square::Marked(x),
            Square::Marked(_) => self,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct BingoBoard {
    board: [[Square; 5]; 5],
}

impl BingoBoard {
    pub fn new() -> BingoBoard {
        BingoBoard {
            board: [[Square::new();5];5]
        }
    }

    pub fn from_chunks(chunks: &[usize]) -> BingoBoard {

        let board = [
            [ Square::from(chunks[0]),Square::from(chunks[1]),Square::from(chunks[2]),Square::from(chunks[3]),Square::from(chunks[4])],
            [ Square::from(chunks[5]),Square::from(chunks[6]),Square::from(chunks[7]),Square::from(chunks[8]),Square::from(chunks[9])],
            [ Square::from(chunks[10]),Square::from(chunks[11]),Square::from(chunks[12]),Square::from(chunks[13]),Square::from(chunks[14])],
            [ Square::from(chunks[15]),Square::from(chunks[16]),Square::from(chunks[17]),Square::from(chunks[18]),Square::from(chunks[19])],
            [ Square::from(chunks[20]),Square::from(chunks[21]),Square::from(chunks[22]),Square::from(chunks[23]),Square::from(chunks[24])],
        ];

        BingoBoard { board }
    }
}

fn main() -> Result<(), Box<dyn Error + 'static + Send + Sync>> {

    let mut contents = String::new();
    let mut f = File::open("input.txt")?;
    f.read_to_string(&mut contents)?;
    let mut iter = contents.split_whitespace();

    let draws: Vec<usize> = iter.by_ref()
        .take(1)
        .flat_map(|s| s.split(','))
        .map(|s| s.parse().unwrap())
        .collect();

    let board_nums: Vec<usize> = iter.map(|s| s.parse().unwrap()).collect();

    let mut boards = Vec::new();
    for chunk in board_nums.chunks(25) {
        if chunk.len() != 25 { break; }
        boards.push(BingoBoard::from_chunks(chunk));
    }
    println!("{:?}", boards);
    //let mut lines = std::io::BufReader::new(File::open("input.txt")?).lines();

    //let lines: Vec<String> = lines.map(|s| s.unwrap())
    //    .filter(|s| !s.is_empty())
    //    .collect();

    //let mut iter = lines.iter();
    //let draws: Vec<usize> = lines[0].split(',')
    //    .map(|s| s.parse().unwrap())
    //    .collect();

    //for chunk in lines[1..].chunks(5) {
    //    if chunk.len() != 5 { break; }
    //    
    //    println!("{:#?}", chunk);
    //}
    //let boards: Vec<BingoBoard> = lines.filter(||);
    Ok(())

}
