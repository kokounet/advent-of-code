use anyhow::Error;
use regex::Regex;
use std::fs;

fn part1(lines: &[&str]) -> u32 {
    lines
        .into_iter()
        .map(|line| {
            let first = line.chars().find_map(|c| c.to_digit(10)).unwrap();
            let last = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
            10 * first + last
        })
        .sum()
}

fn part2(lines: &[&str]) -> u32 {
    let regex_fw = Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let regex_bw = Regex::new(r"([1-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    lines
        .into_iter()
        .map(|line| {
            let enil: String = line.chars().rev().collect(); // line backwards 🤪
            let first = regex_fw
                .find(line)
                .map(|val| match val.as_str() {
                    "1" | "one" => 1,
                    "2" | "two" => 2,
                    "3" | "three" => 3,
                    "4" | "four" => 4,
                    "5" | "five" => 5,
                    "6" | "six" => 6,
                    "7" | "seven" => 7,
                    "8" | "eight" => 8,
                    "9" | "nine" => 9,
                    _ => unreachable!(),
                })
                .unwrap();
            let last = regex_bw
                .find(&enil)
                .map(|val| match val.as_str() {
                    "1" | "eno" => 1,
                    "2" | "owt" => 2,
                    "3" | "eerht" => 3,
                    "4" | "ruof" => 4,
                    "5" | "evif" => 5,
                    "6" | "xis" => 6,
                    "7" | "neves" => 7,
                    "8" | "thgie" => 8,
                    "9" | "enin" => 9,
                    _ => unreachable!(),
                })
                .unwrap();
            10 * first + last
        })
        .sum()
}

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("day01/input.txt")?;
    let lines: Vec<_> = content.lines().collect();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
    Ok(())
}
