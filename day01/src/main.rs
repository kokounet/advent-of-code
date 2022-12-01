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
    let elfs: Vec<Vec<u32>> = content.split("\n").fold(vec![vec![]], |mut acc, curr| {
        match curr.trim().parse() {
            Ok(calories) => acc.last_mut().unwrap().push(calories),
            Err(_) => acc.push(Vec::new()),
        }
        acc
    });
    let max = part1(&elfs);
    let top3 = part2(&elfs);
    println!("{max}");
    println!("{top3}");
    Ok(())
}
