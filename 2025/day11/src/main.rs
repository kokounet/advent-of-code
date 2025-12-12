use std::collections::HashMap;

use anyhow::{anyhow, Result};
use common::time;

fn main() -> Result<()> {
    let content = std::fs::read_to_string("day11/input.txt")?;
    let graph = content
        .lines()
        .map(|line| {
            let (key, rest) = line
                .split_once(':')
                .ok_or(anyhow!("missing `:`: `{line}`"))?;
            let rest = rest.trim();
            let children: Vec<_> = rest.split_ascii_whitespace().collect();
            Ok((key, children))
        })
        .collect::<Result<HashMap<_, _>>>()?;
    println!("{}", time!(part1(&graph)));
    println!("{}", time!(part2(&graph)));
    Ok(())
}

fn part1(graph: &HashMap<&str, Vec<&str>>) -> u64 {
    fn solve<'a>(
        from: &'a str,
        to: &'a str,
        graph: &HashMap<&'a str, Vec<&'a str>>,
        memo: &mut HashMap<(&'a str, &'a str), u64>,
    ) -> u64 {
        if let Some(&cache) = memo.get(&(from, to)) {
            return cache;
        }
        if from == to {
            return 1;
        }
        let mut total = 0;
        for child in graph[from].iter() {
            let paths = solve(child, to, graph, memo);
            memo.insert((child, to), paths);
            total += paths;
        }
        memo.insert((from, to), total);
        total
    }

    let mut memo = HashMap::new();
    solve("you", "out", graph, &mut memo)
}

fn part2(graph: &HashMap<&str, Vec<&str>>) -> u64 {
    fn solve<'a>(
        from: &'a str,
        to: &'a str,
        graph: &HashMap<&'a str, Vec<&'a str>>,
        (fft, dac): (bool, bool),
        memo: &mut HashMap<(&'a str, &'a str, bool, bool), u64>,
    ) -> u64 {
        if let Some(&cache) = memo.get(&(from, to, fft, dac)) {
            return cache;
        }
        if from == to {
            return (fft && dac) as u64;
        }
        let fft = fft || from == "fft";
        let dac = dac || from == "dac";
        let mut total = 0;
        for child in graph[from].iter() {
            let paths = solve(child, to, graph, (fft, dac), memo);
            memo.insert((child, to, fft, dac), paths);
            total += paths;
        }
        memo.insert((from, to, fft, dac), total);
        total
    }

    let mut memo = HashMap::new();
    solve("svr", "out", graph, (false, false), &mut memo)
}
