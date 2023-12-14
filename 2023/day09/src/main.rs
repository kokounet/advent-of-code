use anyhow::Error;
use std::fs;

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("day09/input.txt")?;
    let histories: Vec<_> = content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    println!("{}", part1(&histories));
    println!("{}", part2(&histories));
    Ok(())
}

fn part1(histories: &[Vec<i32>]) -> i32 {
    let mut sum = 0;
    for history in histories {
        let mut diffs = vec![history.clone()];
        while let Some(prev) = diffs.last() {
            let new: Vec<_> = prev.windows(2).map(|win| win[1] - win[0]).collect();
            if new.iter().all(|num| *num == 0) {
                break;
            }
            diffs.push(new)
        }
        let mut prediction = 0;
        for diff in diffs.iter().rev() {
            prediction += diff.last().unwrap();
        }
        sum += prediction;
    }
    sum
}

fn part2(histories: &[Vec<i32>]) -> i32 {
    let mut sum = 0;
    for history in histories {
        let mut diffs = vec![history.clone()];
        while let Some(prev) = diffs.last() {
            let new: Vec<_> = prev.windows(2).map(|win| win[1] - win[0]).collect();
            if new.iter().all(|num| *num == 0) {
                break;
            }
            diffs.push(new)
        }
        let mut prediction = 0;
        for diff in diffs.iter().rev() {
            prediction = diff.first().unwrap() - prediction;
        }
        sum += prediction;
    }
    sum
}
