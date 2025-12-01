use std::fs;

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let content = fs::read_to_string("day01/input.txt")?;
    let numbers = content.lines().map(|line| {
        let line = line.trim();
        let sign = if line.starts_with("R") {
            1
        } else if line.starts_with("L") {
            -1
        } else {
            return Err(anyhow!("Unexpected symbol {}", &line[0..1]));
        };
        let num = line[1..].parse::<i32>()?;
        Ok(sign*num)
    }).collect::<Result<Vec<_>>>()?;
    println!("{}", part1(&numbers));
    println!("{}", part2(&numbers));
    Ok(())
}

fn part1(rots: &[i32]) -> i32 {
    rots.iter().fold((0, 50), |(mut count, mut dial), rot| {
        dial = (dial + rot).rem_euclid(100);
        count += (dial == 0) as i32;
        (count, dial)
    }).0
}


fn part2(rots: &[i32]) -> i32 {
    rots.iter().fold((0, 50), |(mut count, mut dial), rot| {
        let sign = rot.signum();
        let clicks = rot.abs();
        for _ in 0..clicks {
            dial = (dial + sign).rem_euclid(100);
            count += (dial == 0) as i32;
        }
        (count, dial)
    }).0
}