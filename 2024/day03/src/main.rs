use std::fs;

use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let program = fs::read_to_string("day03/input.txt")?;
    println!("{}", part1(&program));
    println!("{}", part2(&program));
    Ok(())
}

fn part1(program: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(program)
        .map(|caps| {
            let left = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let right = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            left * right
        })
        .sum()
}

fn part2(program: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(don't)\(\)|(do)\(\)").unwrap();
    let mut enabled = true; // program starts enabled
    re.captures_iter(program).fold(0, |sum, caps| {
        if caps.get(3).is_some() {
            enabled = false;
            sum
        } else if caps.get(4).is_some() {
            enabled = true;
            sum
        } else if enabled {
            let left = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let right = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            sum + left * right
        } else {
            sum
        }
    })
}
