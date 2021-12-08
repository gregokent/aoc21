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
    board: [(usize, bool); 25],
    won: bool,
}

impl BingoBoard {
    pub fn new() -> BingoBoard {
        BingoBoard {
            board: [(0, false); 25],
            won: false,
        }
    }

    pub fn from_chunks(chunks: &[usize]) -> BingoBoard {

        let board = [
             (chunks[0],false),(chunks[1],false),(chunks[2],false),(chunks[3],false),(chunks[4],false),
             (chunks[5],false),(chunks[6],false),(chunks[7],false),(chunks[8],false),(chunks[9],false),
             (chunks[10],false),(chunks[11],false),(chunks[12],false),(chunks[13],false),(chunks[14],false),
             (chunks[15],false),(chunks[16],false),(chunks[17],false),(chunks[18],false),(chunks[19],false),
             (chunks[20],false),(chunks[21],false),(chunks[22],false),(chunks[23],false),(chunks[24],false),
        ];

        BingoBoard { board , won: false}
    }

    pub fn check_draw(&mut self, draw: usize) {
        if let Some(mut spot) = self.board.iter_mut().find(|x| x.0 == draw) {
            spot.1 = true;
        }
    }

    pub fn check_win(&self) -> bool {
        for index in (0..self.board.len()).step_by(5) {
            if  self.board[index].1 &&
                self.board[index+1].1 &&
                self.board[index+2].1 &&
                self.board[index+3].1 &&
                self.board[index+4].1 {
                    return true
                }
        }

        for index in (0..self.board.len()).take(5) {
            if self.board[index].1 &&
                self.board[index+5].1 &&
                self.board[index+10].1 &&
                self.board[index+15].1 &&
                self.board[index+20].1 {
                    return true;
                }
        }

        false
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

    'outer: for draw in draws.iter() {
        for mut board in &mut boards {
            board.check_draw(*draw);
            if board.check_win() && !board.won {
                println!("board wins!");
                board.won = true;
                let sum = board.board.iter().filter(|(_, marked)| *marked == false).fold(0, |acc, (num, _)| acc + num);
                println!("sum = {}", sum * draw);
            }
        }
    }
    Ok(())

}
