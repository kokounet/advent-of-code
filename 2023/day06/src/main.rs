use anyhow::Error;
use std::fs;

#[derive(Debug)]
struct Record {
    pub time: u64,
    pub dist: u64,
}

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("day06/input.txt")?;
    let lines: Vec<_> = content.lines().collect();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
    Ok(())
}

fn part1(lines: &[&str]) -> u64 {
    let mut iter = lines.into_iter();
    let times = iter
        .next()
        .unwrap()
        .trim_start_matches("Time:")
        .split_whitespace()
        .filter_map(|num| num.parse::<u64>().ok());
    let distances = iter
        .next()
        .unwrap()
        .trim_start_matches("Distance:")
        .split_whitespace()
        .filter_map(|num| num.parse::<u64>().ok());
    let leaderboard: Vec<Record> = times
        .zip(distances)
        .map(|(time, dist)| Record { time, dist })
        .collect();

    leaderboard
        .into_iter()
        .map(|record| number_of_ways_to_beat(record.time, record.dist))
        .fold(1, |acc, curr| acc * curr)
}

fn part2(lines: &[&str]) -> u64 {
    let mut iter = lines.into_iter();
    let time: u64 = iter
        .next()
        .unwrap()
        .trim_start_matches("Time:")
        .replace(|c: char| c.is_whitespace(), "")
        .parse()
        .unwrap();
    let distance: u64 = iter
        .next()
        .unwrap()
        .trim_start_matches("Distance:")
        .replace(|c: char| c.is_whitespace(), "")
        .parse()
        .unwrap();
    number_of_ways_to_beat(time, distance)
}

fn number_of_ways_to_beat(time: u64, best: u64) -> u64 {
    let a = -1.0;
    let b = time as f64;
    let c = -(best as f64);
    debug_assert!(b * b > 4.0 * a * c);
    let delta = (b * b - 4.0 * a * c).sqrt();
    let hold_min = (((-b + delta) / (2.0 * a)) + 1.0).floor() as u64;
    let hold_max = ((-b - delta) / (2.0 * a) - 1.0).ceil() as u64;
    hold_max - hold_min + 1
}
