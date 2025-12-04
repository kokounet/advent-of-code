use std::fs;

use anyhow::{anyhow, Result};
use common::time;

fn main() -> Result<()> {
    let content = fs::read_to_string("day03/input.txt")?;
    let banks = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .map(|d| d as u64)
                        .ok_or(anyhow!("Invalid joltage: {c}"))
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;
    println!("{}", time!(part1(&banks)));
    println!("{}", time!(part2(&banks, 12)));
    Ok(())
}

fn part1(banks: &[Vec<u64>]) -> u64 {
    banks
        .iter()
        .map(|bank| {
            let bank = bank.as_slice();
            let len = bank.len();
            let (i, first) = bank[..len - 1]
                .iter()
                .argmax()
                .expect("bank should not be empty");
            let (_j, last) = bank[i + 1..]
                .iter()
                .argmax()
                .expect("bank should not be empty");
            10 * first + last
        })
        .sum()
}

fn part2(banks: &[Vec<u64>], num: usize) -> u64 {
    banks
        .iter()
        .map(|bank| {
            let mut bank = bank.as_slice();
            let mut res = 0;
            assert!(num < bank.len());
            for k in (1..=num).rev() {
                let (i, joltage) = bank[..bank.len() - k + 1]
                    .iter()
                    .argmax()
                    .expect("bank should not be empty");
                bank = &bank[i + 1..];
                res = 10 * res + joltage;
            }
            res
        })
        .sum()
}

trait ArgMax: Iterator {
    fn argmax(self) -> Option<(usize, Self::Item)>
    where
        Self::Item: Ord,
        Self: Sized,
    {
        // using a manual for-loop here because `Iterator::max` returns the
        // last element in case of equality and this problem requires the first
        let mut it = self.enumerate();
        let mut res = it.next()?;
        for e in it {
            if e.1 > res.1 {
                res = e;
            }
        }
        Some(res)
    }
}

impl<I: Iterator> ArgMax for I {}
