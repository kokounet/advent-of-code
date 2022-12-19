use anyhow::Result;
use pathfinding::prelude::dijkstra_all;
use regex::Regex;
use std::collections::{BinaryHeap, HashMap};

fn main() -> Result<()> {
    let content = std::fs::read_to_string("day16/example.txt")?;
    let (valves, tunnels) = parse(&content);
    println!("{valves:?}");
    println!("{tunnels:?}");
    println!("{}", part1(&valves, &tunnels));
    Ok(())
}

fn part1(valves: &HashMap<String, i64>, tunnels: &HashMap<String, Vec<String>>) -> i64 {
    let mut remaining = 30;
    let mut start = "AA".to_string();
    let reachables = dijkstra_all(&start, |node| {
        tunnels[node]
            .iter()
            .map(|neighbor| (neighbor.clone(), 1))
            .collect::<Vec<_>>()
    });
    println!("{reachables:?}");
    let (next, flow) = valves
        .iter()
        .max_by_key(|&(valve, &flow)| flow * (remaining - reachables[valve].1 - 1))
        .unwrap();
    println!("{next}: {flow}");
    0
}

fn parse(content: &str) -> (HashMap<String, i64>, HashMap<String, Vec<String>>) {
    let re =
        Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (\w+(?:, \w+)*)")
            .unwrap();
    content
        .lines()
        .map(|l| re.captures(l).unwrap())
        .filter_map(|cap| Some((cap.get(1)?, cap.get(2)?, cap.get(3)?)))
        .fold(
            (HashMap::new(), HashMap::new()),
            |(mut verts, mut edges), (name, flow, tunnels)| {
                let name = name.as_str().to_string();
                let flow = flow.as_str().parse().unwrap();
                if flow > 0 {
                    verts.insert(name.clone(), flow);
                }
                edges.insert(
                    name,
                    tunnels.as_str().split(", ").map(str::to_owned).collect(),
                );
                (verts, edges)
            },
        )
}
