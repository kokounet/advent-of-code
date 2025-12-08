use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};
use common::time;

fn main() -> Result<()> {
    let content = std::fs::read_to_string("day07/input.txt")?;
    let mut it = content.lines();
    let start = it
        .next()
        .ok_or(anyhow!("file can't be empty"))?
        .chars()
        .enumerate()
        .find_map(|(i, c)| (c == 'S').then_some(i as i64))
        .ok_or(anyhow!("first line should contain an `S`"))?;
    let manifolds: Vec<HashSet<_>> = it
        .filter_map(|line| {
            let set: HashSet<_> = line
                .chars()
                .enumerate()
                .filter_map(move |(c, e)| (e == '^').then_some(c as i64))
                .collect();
            (!set.is_empty()).then_some(set)
        })
        .collect();
    println!("{}", time!(part1(start, &manifolds)));
    println!("{}", time!(part2(start, &manifolds)));
    Ok(())
}

fn part1(start: i64, manifolds: &[HashSet<i64>]) -> i64 {
    fn step(beams: HashSet<i64>, manifolds: &HashSet<i64>) -> (i64, HashSet<i64>) {
        let mut res = HashSet::with_capacity(beams.len());
        let mut splits = 0;
        for beam in beams {
            if manifolds.contains(&beam) {
                res.insert(beam - 1);
                res.insert(beam + 1);
                splits += 1;
            } else {
                res.insert(beam);
            }
        }
        (splits, res)
    }

    let mut state = HashSet::from_iter(std::iter::once(start));
    let mut splits = 0;
    for manifolds in manifolds.iter() {
        let (split, new) = step(state, manifolds);
        splits += split;
        state = new;
    }
    splits
}

fn part2(start: i64, manifolds: &[HashSet<i64>]) -> i64 {
    fn solve(
        pos: (i64, i64),
        manifolds: &[HashSet<i64>],
        memo: &mut HashMap<(i64, i64), i64>,
    ) -> i64 {
        if let Some(cache) = memo.get(&pos) {
            return *cache;
        }
        let Some((current, rest)) = manifolds.split_first() else {
            return 1;
        };
        let (x, y) = pos;
        let res = if current.contains(&x) {
            solve((x - 1, y + 1), rest, memo) + solve((x + 1, y + 1), rest, memo)
        } else {
            solve((x, y + 1), rest, memo)
        };
        memo.insert(pos, res);
        res
    }

    let mut memo = HashMap::new();
    let res = solve((start, 0), manifolds, &mut memo);
    res
}
