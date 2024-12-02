use std::collections::BTreeMap;
use std::fs;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let content = fs::read_to_string("day01/input.txt")?;
    let (left, right) = content
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(" ").context("line should have space")?;
            let l = l.trim().parse::<i32>()?;
            let r = r.trim().parse::<i32>()?;
            Ok((l, r))
        })
        .collect::<Result<(Vec<_>, Vec<_>)>>()?; // wtf rust ??? this works ???
    println!("{}", part1(left.clone(), right.clone()));
    println!("{}", part2(&left, &right));
    Ok(())
}

fn part1(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn part2(left: &[i32], right: &[i32]) -> i32 {
    let left = count(left);
    let right = count(right);
    left.into_iter()
        .map(|(e, l_count)| {
            let r_count = *right.get(&e).unwrap_or(&0);
            e * l_count * r_count
        })
        .sum()
}

fn count<'a>(iter: impl IntoIterator<Item = &'a i32>) -> BTreeMap<i32, i32> {
    let mut res = BTreeMap::new();
    for e in iter.into_iter() {
        *res.entry(*e).or_default() += 1;
    }
    res
}
