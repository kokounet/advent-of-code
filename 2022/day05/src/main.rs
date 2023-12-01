use anyhow::Error;
use regex::Regex;
use std::fs;

type Stack = Vec<char>;

#[derive(Debug, Clone, Copy)]
struct Command {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}

fn parse_stacks(stacks: &[&str]) -> Vec<Stack> {
    let mut iter = stacks.iter().rev();
    let header: Vec<Stack> = iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(|_| Stack::new())
        .collect();
    iter.fold(header, |mut acc, curr| {
        let crates = curr.chars().enumerate().filter(|(_, c)| c.is_alphabetic());
        for (i, c) in crates {
            acc[(i - 1) / 4].push(c);
        }
        acc
    })
}

fn parse_commands(commands: &[&str]) -> Vec<Command> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    commands
        .iter()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            Command {
                amount: cap[1].parse().unwrap(),
                from: cap[2].parse::<usize>().unwrap() - 1,
                to: cap[3].parse::<usize>().unwrap() - 1,
            }
        })
        .collect()
}

fn parse(content: &str) -> (Vec<Stack>, Vec<Command>) {
    let lines: Vec<_> = content.lines().collect();
    let mut split = lines.split(|line| line.is_empty());
    let stacks = split.next().unwrap();
    let commands = split.next().unwrap();
    (parse_stacks(stacks), parse_commands(commands))
}

fn part1(stacks: Vec<Stack>, commands: &[Command]) -> String {
    let reorg = commands.iter().fold(stacks, |mut stacks, c| {
        for _ in 0..c.amount {
            let krate = stacks[c.from].pop().unwrap();
            stacks[c.to].push(krate);
        }
        stacks
    });
    reorg.iter().map(|stack| stack.last().unwrap()).collect()
}

fn part2(stacks: Vec<Stack>, commands: &[Command]) -> String {
    let reorg = commands.iter().fold(stacks, |mut stacks, c| {
        let origin = &mut stacks[c.from];
        let krates: Vec<_> = origin.drain((origin.len() - c.amount)..).collect();
        stacks[c.to].extend(krates);
        stacks
    });
    reorg.iter().map(|stack| stack.last().unwrap()).collect()
}

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("day05/input.txt")?;
    let (stacks, commands) = parse(&content);
    // println!("{commands:?}");
    println!("{}", part1(stacks.clone(), &commands));
    println!("{}", part2(stacks, &commands));
    Ok(())
}
