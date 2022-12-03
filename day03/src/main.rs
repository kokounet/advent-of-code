use anyhow::Error;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn part1(lines: &[String], priority: &HashMap<char, u32>) -> u32 {
    lines
        .iter()
        .map(|sack| {
            let (c1, c2) = sack.split_at(sack.len() / 2);
            let c1 = HashSet::<char>::from_iter(c1.chars());
            let c2 = HashSet::<char>::from_iter(c2.chars());
            priority[c1.intersection(&c2).next().unwrap()]
        })
        .sum()
}

fn badge(group: &[HashSet<char>]) -> char {
    let mut group = group.iter();
    let mut badge = group.next().unwrap().clone();
    for sack in group {
        badge.retain(|c| sack.contains(c))
    }
    badge.into_iter().next().unwrap()
}

fn part2(lines: &[String], priority: &HashMap<char, u32>) -> u32 {
    let sacks: Vec<_> = lines
        .iter()
        .map(|line| HashSet::<char>::from_iter(line.chars()))
        .collect();
    sacks.chunks(3).map(badge).map(|c| priority[&c]).sum()
}

fn main() -> Result<(), Error> {
    let content: Vec<_> = fs::read_to_string("day03/input.txt")?
        .lines()
        .map(ToOwned::to_owned)
        .collect();
    let priority: HashMap<_, _> = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, l)| (l, (i + 1) as u32))
        .collect();

    println!("{}", part1(&content, &priority));
    println!("{}", part2(&content, &priority));
    Ok(())
}
