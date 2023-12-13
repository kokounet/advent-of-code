use anyhow::Error;
use regex::Regex;
use std::fs;

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("day03/input.txt")?;
    let schema: Vec<_> = content.lines().collect();
    println!("{}", part1(&schema));
    Ok(())
}

fn part1(schema: &[&str]) -> u32 {
    let height = schema.len();
    let regex = Regex::new(r"(\d+)").unwrap();
    let numbers: Vec<_> = schema
        .into_iter()
        .enumerate()
        .flat_map(|(row, line)| {
            let row_start = (row as isize - 1).max(0) as usize;
            let row_end = (row + 1).min(height - 1);
            let width = line.len();
            regex.find_iter(line).map(move |found| {
                let num = found.as_str().parse::<u32>().unwrap();
                let col_start = (found.start() as isize - 1).max(0) as usize;
                let col_end = (found.end() + 1).min(width);
                (num, (row_start..=row_end, col_start..col_end))
            })
        })
        .collect();

    numbers
        .into_iter()
        .filter_map(move |(num, (rows, cols))| {
            // println!("{num}: ({rows:?}, {cols:?})");
            schema[rows]
                .iter()
                .any(|line| {
                    line[cols.clone()]
                        .chars()
                        .any(|c| !c.is_digit(10) && c != '.')
                })
                .then_some(num)
        })
        .sum()
}
