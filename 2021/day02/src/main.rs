use std::fs::File;
use std::io::{BufRead, BufReader};

enum Direction {
    Up,
    Down,
    Forward,
}

fn solution02(file: &str) -> Result<i64, std::io::Error> {
    let (pos, depth, _) = {
        let file = File::open(file)?;
        BufReader::new(file)
            .lines()
            .into_iter()
            .map(|line| {
                // parse the line
                let line = line.unwrap();
                let mut frags = line.split(" ");
                let direction = match frags.next().unwrap() {
                    "forward" => Direction::Forward,
                    "up" => Direction::Up,
                    "down" => Direction::Down,
                    _ => panic!("Unknown command in file"),
                };
                let amount = frags.next().unwrap().parse::<i64>().unwrap();
                (direction, amount)
            })
            .fold((0, 0, 0), |(pos, depth, aim), (dir, amount)| match dir {
                // down increase aim
                Direction::Down => (pos, depth, aim + amount), 
                // up decrease  aim
                Direction::Up => (pos, depth, aim - amount), 
                // forward increase pos and depth
                Direction::Forward => (pos + amount, depth + aim * amount, aim),
            })
    };
    Ok(pos * depth)
}

fn main() -> Result<(), std::io::Error> {
    println!("{}", solution02("input.txt")?);
    Ok(())
}
