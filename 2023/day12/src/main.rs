use anyhow::Error;
use std::fs;

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("day12/input.txt")?;
    let records = content
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            // replace any contiguous sequence of '.' by a single one
            let collapsed: Vec<_> = split
                .next()
                .unwrap()
                .split('.')
                .filter(|slice| !slice.is_empty())
                .collect();
            let record = collapsed
                .join(".")
                .chars()
                .map(|c| match c {
                    '.' => Status::OK,
                    '#' => Status::KO,
                    '?' => Status::NA,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();
            let constraints = split
                .next()
                .unwrap()
                .split(',')
                .map(|num| num.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            (record, constraints)
        })
        .collect::<Vec<_>>();
    println!("{}", part1(records));
    Ok(())
}

fn part1(records: Vec<(Vec<Status>, Vec<u32>)>) -> u32 {
    records
        .into_iter()
        .map(|(record, constraint)| solve(record, constraint))
        .sum()
}

fn solve(mut record: Vec<Status>, constraint: Vec<u32>) -> u32 {
    let mut solutions = 0;
    if let Some(na_pos) = record
        .iter()
        .enumerate()
        .rev()
        .find_map(|(i, status)| matches!(status, Status::NA).then_some(i))
    {
        record[na_pos] = Status::OK;
        solutions += solve(record.clone(), constraint.clone());
        record[na_pos] = Status::KO;
        solutions += solve(record, constraint);
    } else if check(&record, &constraint) {
        solutions += 1;
    }

    solutions
}

fn check(record: &[Status], constraint: &[u32]) -> bool {
    assert!(record
        .into_iter()
        .all(|status| !matches!(status, Status::NA)));
    let collapsed: Vec<_> = record
        .split(|status| matches!(status, Status::OK))
        .filter_map(|slice| match slice.len() {
            0 => None,
            count => Some(count as u32),
        })
        .collect();
    collapsed.len() == constraint.len()
        && collapsed
            .iter()
            .zip(constraint)
            .all(|(left, right)| left == right)
}

fn debug(record: &[Status]) -> String {
    record
        .iter()
        .map(|status| match status {
            Status::OK => '.',
            Status::KO => '#',
            Status::NA => '?',
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    OK,
    KO,
    NA,
}
