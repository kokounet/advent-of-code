use anyhow::{anyhow, Result};
use common::time;
use itertools::Itertools;

fn main() -> Result<()> {
    let content = std::fs::read_to_string("day09/example.txt")?;
    let points = content
        .lines()
        .map(|line| {
            let (x, y) = line
                .split_once(',')
                .ok_or(anyhow!("missing comma: {line}"))?;
            let x = x.parse::<i64>()?;
            let y = y.parse::<i64>()?;
            Ok([x, y])
        })
        .collect::<Result<Vec<_>>>()?;
    println!("{}", time!(part1(&points)));
    //println!("{}", time!(part2(&points)));
    Ok(())
}

fn area(a: &[i64; 2], b: &[i64; 2]) -> i64 {
    let w = (b[0] - a[0]).abs() + 1;
    let h = (b[1] - a[1]).abs() + 1;
    w * h
}

fn part1(points: &[[i64; 2]]) -> i64 {
    points
        .iter()
        .combinations(2)
        .map(|ps| {
            let a = ps[0];
            let b = ps[1];
            area(a, b)
        })
        .max()
        .unwrap()
}
