use anyhow::Error;
use std::{fs, ops::RangeInclusive};

fn part1(pairs: &[[RangeInclusive<i32>; 2]]) -> u32 {
    pairs
        .iter()
        .filter(|[first, second]| {
            (second.start() - first.start()) * (second.end() - first.end()) <= 0
        })
        .count() as u32
}

fn part2(pairs: &[[RangeInclusive<i32>; 2]]) -> u32 {
    pairs
        .iter()
        .filter(|[first, second]| {
            first.start().max(second.start()) <= first.end().min(second.end())
        })
        .count() as u32
}

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("day04/input.txt")?;
    let pairs: Vec<_> = content
        .lines()
        .filter_map(|line| {
            let mut pair = line.split(',').filter_map(|section| {
                let mut iter = section.split('-');
                let start: i32 = iter.next()?.parse().ok()?;
                let stop: i32 = iter.next()?.parse().ok()?;
                Some(start..=stop)
            });
            Some([pair.next()?, pair.next()?])
        })
        .collect();
    println!("{}", part1(&pairs));
    println!("{}", part2(&pairs));
    Ok(())
}
