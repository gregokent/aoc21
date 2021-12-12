
use std::error::Error;
use std::fs::File;
use std::io::{BufReader,BufRead};
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug,Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: u32,
    y: u32
}

impl Point {
    pub fn new() -> Point {
        Point {
            x: 0,
            y: 0,
        }
    }

    pub fn from_input(input: &str) -> Result<Point, Box<dyn Error + 'static + Send + Sync>> {
        if let Some((x, y)) = input.split_once(",") {
            Ok(Point {
                    x: x.parse()?,
                    y: y.parse()?,
                })

        } else {
            Err("invalid input".into())
        }
    }
}

fn parse_points(line: &str) -> Result<(Point, Point), Box<dyn Error + 'static + Send + Sync>> {
    if let Some((point_a, point_b)) =  line.split_once(" -> ") {
        let point_a = Point::from_input(point_a)?;
        let point_b = Point::from_input(point_b)?;

        Ok((point_a, point_b))
    } else {
        Err(format!("'{}' is invalid input", line).into())
    }

}
fn is_horizontal(a: Point, b: Point) -> bool {
    a.y == b.y
}

fn is_vertical(a: Point, b: Point) -> bool {
    a.x == b.x
}

fn is_horizontal_or_vertical(a: Point, b: Point) -> bool {
    if is_horizontal(a,b) || is_vertical(a,b) { 
        true
    } else {
        false
    }
}

fn main() -> Result<(), Box<dyn Error + 'static + Send + Sync>> {

    let file = File::open("input.txt")?;
    let bufr = BufReader::new(file);
    let lines = bufr.lines();
    
    let points: Vec<(Point,Point)> = lines.map(|line| parse_points(&line.unwrap()).unwrap() ).filter(|(a,b)| is_horizontal_or_vertical(*a,*b)).collect();
    println!("{}", points.len());

    let mut map = HashMap::new();

    for (point_a, point_b) in points {
       if is_vertical(point_a, point_b) {
           let range = if point_a.y < point_b.y { point_a.y..=point_b.y } else { point_b.y..=point_a.y };
           for py in range {
               map.entry(Point { x: point_a.x, y: py } )
                   .and_modify(|e| { *e += 1 })
                   .or_insert(1);
           }

       } else if is_horizontal(point_a, point_b) {
           let range = if point_a.x < point_b.x { point_a.x..=point_b.x } else { point_b.x..=point_a.x };
           for px in range {
               map.entry(Point { x: px, y: point_a.y } )
                   .and_modify(|e| { *e += 1 })
                   .or_insert(1);
           }

       }
    }

    let npoints = map.iter().filter(|(k,v)| **v > 1).count();

    println!("{}", npoints);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "143,592 -> 143,296";

    #[test]
    fn split_by_slice() {
        match INPUT.split_once(" -> ") {
            Some((left, right)) => { 
                assert_eq!(left, "143,592");
                assert_eq!(right, "143,296");
            },
            None => unreachable!()
        }
    }

    #[test]
    fn parse_point() {
        let point_text = "143,296";

        let point = Point::from_input(point_text).unwrap();
        assert_eq!(point, Point { x: 143, y: 296 });
    }

    #[test]
    fn parse_input() -> Result<(), Box<dyn Error + 'static + Send + Sync>>{
        let (point_a, point_b) = parse_points(INPUT)?;
        assert_eq!(point_a, Point { x: 143, y: 592 });
        assert_eq!(point_b, Point { x: 143, y: 296 });

        Ok(())
    }

    #[test]
    fn point_is_horizontal() {
        assert!(is_horizontal_or_vertical(Point { x: 101, y: 150 }, Point { x: 190, y: 150 }));
    }

    #[test]
    fn point_is_vertical() {
        assert!(is_horizontal_or_vertical(Point { x: 101, y: 150 }, Point { x: 101, y: 190 }));
    }

}
