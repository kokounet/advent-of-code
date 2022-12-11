use anyhow::Result;
use num::integer::lcm;
use std::collections::VecDeque;
use std::fs;

#[derive(Debug, Clone, Copy)]
enum Op {
    Add(u64),
    Mul(u64),
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    pub items: VecDeque<u64>,
    pub operation: Op,
    pub test: u64,
    pub success: usize,
    pub failure: usize,
}

fn main() -> Result<()> {
    let content = fs::read_to_string("day11/input.txt")?;
    let lines: Vec<_> = content.lines().map(|l| l.trim()).collect();
    let monkeys: Vec<_> = lines.split(|l| l.is_empty()).map(Monkey::from).collect();
    println!("{}", part1(monkeys.clone()));
    println!("{}", part2(monkeys));
    Ok(())
}

fn part1(mut monkeys: Vec<Monkey>) -> u64 {
    let mut inspections = vec![0; monkeys.len()];
    for _ in 1..=20 {
        round(&mut monkeys, &mut inspections, |w| w / 3);
    }
    inspections.sort();
    inspections.iter().rev().take(2).product()
}

fn part2(mut monkeys: Vec<Monkey>) -> u64 {
    // compute the lowest common multiplier so that the worriness level doesn't overflow
    let lcm: u64 = monkeys.iter().map(|m| m.test).reduce(lcm).unwrap();
    let mut inspections = vec![0; monkeys.len()];
    for _ in 1..=10_000 {
        round(&mut monkeys, &mut inspections, |w| w % lcm);
    }
    inspections.sort();
    inspections.iter().rev().take(2).product()
}

/// Simulate one round of Keep Away
fn round<F: Fn(u64) -> u64>(monkeys: &mut [Monkey], inspections: &mut [u64], f: F) {
    for i in 0..monkeys.len() {
        let (inspection, throws) = turn(&mut monkeys[i], &f);
        for (j, val) in throws {
            monkeys[j].items.push_back(val);
        }
        inspections[i] += inspection;
    }
}

/// Simulate one turn of monkey, returning the index and value for the throw
fn turn<F: Fn(u64) -> u64>(monkey: &mut Monkey, f: F) -> (u64, Vec<(usize, u64)>) {
    let mut throws = vec![];
    let inspection = monkey.items.len() as u64;
    while let Some(item) = monkey.items.pop_front() {
        let level = f(match monkey.operation {
            Op::Add(amount) => item + amount,
            Op::Mul(amount) => item * amount,
            Op::Square => item * item,
        });
        let id = if level % monkey.test == 0 {
            monkey.success
        } else {
            monkey.failure
        };
        throws.push((id, level));
    }
    (inspection, throws)
}

impl From<&[&str]> for Monkey {
    fn from(spec: &[&str]) -> Self {
        let mut it = spec.iter().skip(1);
        let items = it
            .next()
            .and_then(|line| line.strip_prefix("Starting items: "))
            .map(|list| list.split(", ").map(|p| p.parse().unwrap()).collect())
            .unwrap();
        let operation = it
            .next()
            .and_then(|line| line.strip_prefix("Operation: new = old "))
            .map(|op| op.split_whitespace().collect::<Vec<_>>())
            .map(|op| match op.as_slice() {
                ["*", "old"] => Op::Square,
                ["*", val] => Op::Mul(val.parse().unwrap()),
                ["+", val] => Op::Add(val.parse().unwrap()),
                [..] => unreachable!(),
            })
            .unwrap();
        let test = it
            .next()
            .and_then(|line| line.strip_prefix("Test: divisible by "))
            .map(|val| val.parse().unwrap())
            .unwrap();
        let success = it
            .next()
            .and_then(|line| line.strip_prefix("If true: throw to monkey "))
            .map(|id| id.parse().unwrap())
            .unwrap();
        let failure = it
            .next()
            .and_then(|line| line.strip_prefix("If false: throw to monkey "))
            .map(|id| id.parse().unwrap())
            .unwrap();
        Monkey {
            items,
            operation,
            test,
            success,
            failure,
        }
    }
}
