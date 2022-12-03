use anyhow::Error;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn part1(lines: &[String], priority: &HashMap<char, u32>) -> u32 {
    let mut score = 0;
    for sack in lines {
        let (c1, c2) = sack.split_at(sack.len() / 2);
        let c1 = HashSet::<char>::from_iter(c1.chars());
        let c2 = HashSet::<char>::from_iter(c2.chars());
        for c in c1.intersection(&c2) {
            score += priority[c];
        }
    }
    score
}

fn part2(lines: &[String], priority: &HashMap<char, u32>) -> u32 {
    let mut score = 0;
    for group in lines.chunks(3) {
        let mut group = group
            .iter()
            .map(|sack| HashSet::<char>::from_iter(sack.chars()));
        let mut badge = group.next().unwrap();
        for sack in group {
            badge.retain(|c| sack.contains(c))
        }
        for c in badge {
            score += priority[&c];
        }
    }
    score
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
