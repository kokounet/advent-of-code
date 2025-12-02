use std::{fmt::Write, fs, ops::RangeInclusive, time::Instant};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let content = fs::read_to_string("day02/input.txt")?;
    let ranges = content
        .split(",")
        .map(|range| {
            let (low, high) = range
                .split_once("-")
                .ok_or(anyhow!("unexpected range: {}", range))?;
            let low = low.parse::<u64>()?;
            let high = high.parse::<u64>()?;
            Ok(low..=high)
        })
        .collect::<Result<Vec<_>>>()?;
    let start = Instant::now();
    let p1 = part1(&ranges);
    let end = Instant::now();
    println!("{} ({:?})", p1, end.duration_since(start));

    let start = Instant::now();
    let p2 = part2(&ranges);
    let end = Instant::now();
    println!("{} ({:?})", p2, end.duration_since(start));
    Ok(())
}

fn part1(ranges: &[RangeInclusive<u64>]) -> u64 {
    let mut buf = String::new();
    let mut sum = 0;
    for range in ranges {
        for id in range.clone() {
            buf.clear();
            write!(&mut buf, "{}", id).expect("fail write id into string");
            let digits = buf.as_str();
            let len = digits.len();
            if len & 0x1 == 1 {
                continue;
            } // len is odd, can't be invalid
            let mid = digits.len() / 2;
            sum += id * (digits[0..mid] == digits[mid..]) as u64;
        }
    }
    sum
}

fn part2(ranges: &[RangeInclusive<u64>]) -> u64 {
    let mut buf = String::new();
    let mut sum = 0;
    for range in ranges {
        for id in range.clone() {
            buf.clear();
            write!(&mut buf, "{}", id).expect("fail write id into string");
            let digits = buf.as_str();
            let len = digits.len();
            sum += id * (1..=len / 2).any(|l| check(l, &digits)) as u64;
        }
    }
    sum
}

fn check(k: usize, s: &str) -> bool {
    assert!(k < s.len());
    if s.len() % k != 0 {
        // this length of prefix can not be a match
        return false;
    }
    // n * k = s.len()
    let n = s.len() / k;
    let prefix = &s[0..k];
    (1..n).all(|i| &s[k * i..k * (i + 1)] == prefix)
}
