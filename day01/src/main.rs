use anyhow::Error;
use std::fs;

fn part1(elfs: &Vec<Vec<u32>>) -> u32 {
    elfs.iter().map(|snacks| snacks.iter().sum()).max().unwrap()
}

fn part2(elfs: &Vec<Vec<u32>>) -> u32 {
    let mut total: Vec<_> = elfs.iter().map(|snacks| snacks.iter().sum()).collect();
    total.sort();
    total.iter().rev().take(3).sum()
}

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("day01/input.txt")?;
    let calories: Vec<_> = content.split("\n").map(|a| a.trim().to_string()).collect();
    let elfs: Vec<Vec<u32>> = calories
        .split(|s| s.is_empty())
        .map(|split| split.into_iter().map(|s| s.parse().unwrap()).collect())
        .collect();
    let max = part1(&elfs);
    let top3 = part2(&elfs);
    println!("{max}");
    println!("{top3}");
    Ok(())
}
