use std::collections::{HashSet, VecDeque};

use anyhow::{anyhow, Result};
use common::time;

type Machine = (u32, Vec<u32>, Vec<u32>);

fn main() -> Result<()> {
    let content = std::fs::read_to_string("day10/input.txt")?;
    let machines: Vec<Machine> = content
        .lines()
        .map(|line| {
            let mut it = line.trim().split_ascii_whitespace();
            let diagram = it.next().ok_or(anyhow!("missing light diagram"))?;
            let mut schema = it.collect::<Vec<_>>();
            let joltage = schema.pop().ok_or(anyhow!("missing joltage"))?;

            let diagram = parse::diagram(diagram)?;
            let schema = parse::schema(&schema)?;
            let joltage = parse::joltage(joltage)?;
            Ok((diagram, schema, joltage))
        })
        .collect::<Result<Vec<_>>>()?;

    println!("{}", time!(part1(&machines)));
    Ok(())
}

fn part1(machines: &[Machine]) -> u32 {
    fn solve(machine: &Machine) -> u32 {
        let target = machine.0;
        let transforms = &machine.1;
        let mut explored = HashSet::from([0]);
        let mut queue = VecDeque::from([(0, vec![])]);
        while let Some((state, path)) = queue.pop_front() {
            if state == target {
                return path.len() as u32;
            }

            for &transform in transforms {
                let new = state ^ transform;
                if !explored.insert(new) {
                    continue;
                }
                let mut path = path.clone();
                if new == target {
                    path.push(transform);
                    return path.len() as u32;
                }
                path.push(transform);
                queue.push_back((new, path));
            }
        }
        unreachable!()
    }
    machines.iter().map(|machine| solve(machine)).sum()
}

mod parse {
    use super::*;

    pub fn diagram(diagram: &str) -> Result<u32> {
        if !diagram.starts_with('[') && !diagram.ends_with(']') {
            return Err(anyhow!("wrong light diagram: {diagram}"));
        }
        Ok(diagram[1..diagram.len() - 1]
            .chars()
            .enumerate()
            .fold(0, |state, (i, curr)| {
                let flag = ((curr == '#') as u32) << i;
                state | flag
            }))
    }

    pub fn schema(schema: &[&str]) -> Result<Vec<u32>> {
        schema.iter().map(|wiring| buttons(wiring)).collect()
    }

    fn buttons(wiring: &str) -> Result<u32> {
        if !wiring.starts_with('(') && !wiring.ends_with(')') {
            return Err(anyhow!("wrong buttons: {wiring}"));
        }
        wiring[1..wiring.len() - 1]
            .split(',')
            .try_fold(0, |acc, index| {
                let num = index.parse::<usize>().ok()?;
                Some(acc | 1 << num)
            })
            .ok_or(anyhow!("wrong buttons: {wiring}"))
    }

    pub fn joltage(req: &str) -> Result<Vec<u32>> {
        if !req.starts_with('{') && !req.ends_with('}') {
            return Err(anyhow!("wrong joltage: {req}"));
        }
        req[1..req.len() - 1]
            .split(',')
            .map(|jolt| jolt.parse::<u32>().map_err(|e| anyhow!(e)))
            .collect()
    }
}
