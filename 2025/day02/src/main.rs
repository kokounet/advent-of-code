use std::{fs, ops::{RangeInclusive}};

use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let content = fs::read_to_string("day02/input.txt")?;
    let ranges = content.split(",").map(|range| {
        let (low, high) = range.split_once("-").ok_or(anyhow!("unexpected range: {}", range))?;
        let low = low.parse::<u64>()?;
        let high = high.parse::<u64>()?;
        Ok(low..=high)
    }).collect::<Result<Vec<_>>>()?;
    println!("{}", part1(&ranges));
    println!("{}", part2(&ranges));
    Ok(())
}

fn part1(ranges: &[RangeInclusive<u64>]) -> u64 {
    ranges.iter().flat_map(|range| {
        range.clone().filter_map(|id| {
            let digits = id.to_string();
            let len = digits.len();
            if len & 0x1 == 1 { return None; } // len is odd, can't be invalid
            let mid = digits.len() / 2;
            (digits[0..mid] == digits[mid..]).then_some(id)
        })
    }).sum()
}

fn part2(ranges: &[RangeInclusive<u64>]) -> u64 {
    ranges.iter().flat_map(|range| {
        range.clone().filter_map(|id| {
            let digits = id.to_string();
            let len = digits.len();
            (1..=len/2).any(|l| check(l, &digits)).then_some(id)
        })
    }).sum()
}

fn check(k: usize, s: &str) -> bool {
    assert!(k < s.len());
    if s.len() % k != 0 {
        // this length of prefix can not be a match
        return false;
    }
    // len * k = id.len()
    let n = s.len() / k;
    let prefix = &s[0..k];
    prefix.repeat(n) == s
}