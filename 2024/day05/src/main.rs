use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    fs,
};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let content = fs::read_to_string("day05/input.txt")?;
    let mut lines = content.lines();
    let mut rules = BTreeMap::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (left, right) = line.split_once("|").context("could not split at `|`")?;
        let left = left.parse::<i32>()?;
        let right = right.parse::<i32>()?;
        rules
            .entry(left)
            .or_insert_with(|| BTreeSet::new())
            .insert(right);
    }
    let updates = lines
        .map(|line| {
            line.split(",")
                .map(|e| e.parse::<i32>().context("invalid number"))
                .collect()
        })
        .collect::<Result<Vec<Vec<_>>>>()?;
    println!("{}", part1(&rules, &updates));
    println!("{}", part2(&rules, updates));

    Ok(())
}

fn compare(left: &i32, right: &i32, rules: &BTreeMap<i32, BTreeSet<i32>>) -> Ordering {
    use Ordering::*;
    let before = rules.get(left).map(|r| r.contains(right)).unwrap_or(true); // should be on the left
    let after = !rules.get(right).map(|r| r.contains(left)).unwrap_or(false); // should not be on the right
    if before && after {
        Less
    } else {
        Greater
    }
}

fn part1(rules: &BTreeMap<i32, BTreeSet<i32>>, updates: &[Vec<i32>]) -> i32 {
    updates
        .iter()
        .filter_map(|update| {
            let midpoint = update[update.len() / 2];
            update
                .is_sorted_by(|left, right| compare(left, right, rules).is_le())
                .then_some(midpoint)
        })
        .sum()
}

fn part2(rules: &BTreeMap<i32, BTreeSet<i32>>, mut updates: Vec<Vec<i32>>) -> i32 {
    updates
        .iter_mut()
        .filter_map(|update| {
            if update.is_sorted_by(|left, right| compare(left, right, rules).is_le()) {
                return None;
            }
            update.sort_by(|left, right| compare(left, right, rules));
            Some(update[update.len() / 2])
        })
        .sum()
}
