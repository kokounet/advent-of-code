use std::fs;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let content = fs::read_to_string("day02/input.txt")?;
    let reports = content
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|e| e.parse::<i32>().context("fail to parse int")) // convert to anyhow error
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;
    println!("{}", part1(&reports));
    println!("{}", part2(reports));
    Ok(())
}

/// Returns the left index of the first failure in the report, or `None` if none
/// were found (*i.e* the report is safe).
fn find_unsafe(report: &[i32]) -> Option<usize> {
    fn is_safe(from: &i32, to: &i32, dir: i32) -> bool {
        let diff = to - from;
        dir == diff.signum() && diff.abs() >= 1 && diff.abs() <= 3
    }

    let mut wins = report.windows(2).peekable();
    let dir: i32 = {
        let win = wins.peek().unwrap();
        (win[1] - win[0]).signum()
    };
    wins.enumerate()
        .find_map(|(i, win)| (!is_safe(&win[0], &win[1], dir)).then_some(i))
}

fn part1(reports: &[Vec<i32>]) -> i32 {
    reports
        .into_iter()
        .map(|report| find_unsafe(report).is_none() as i32)
        .sum()
}

fn part2(reports: Vec<Vec<i32>>) -> i32 {
    reports
        .into_iter()
        .map(|report| {
            let Some(i) = find_unsafe(&report) else {
                return true as i32; // safe reports are still safe
            };
            // maybe the first level is a failure because it messes up the direction
            // or left or right of the failure.
            let mut first = report.clone();
            let mut left = report.clone();
            let mut right = report;
            first.remove(0);
            left.remove(i);
            right.remove(i + 1);

            // checks in order of likelyhood
            find_unsafe(&right)
                .and_then(|_| find_unsafe(&left))
                .and_then(|_| find_unsafe(&first))
                .is_none() as i32
        })
        .sum()
}
