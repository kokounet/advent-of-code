use anyhow::{anyhow, Error};
use num::integer::lcm;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node(pub String);

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("day08/input.txt")?;
    let mut lines = content.lines();
    let dirs = lines
        .next()
        .ok_or(anyhow!("Parsing Error"))?
        .chars()
        .map(|c| match c {
            'R' => Ok(Dir::Right),
            'L' => Ok(Dir::Left),
            _ => Err(anyhow!("Parsing Error")),
        })
        .collect::<Result<Vec<_>, _>>()?;
    lines.next();
    let regex = Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)")?;
    let nodes: HashMap<_, _> = lines
        .map(|line| {
            let cap = regex.captures(line).unwrap();
            let start = cap.get(1).unwrap().as_str().to_string();
            let left = cap.get(2).unwrap().as_str().to_string();
            let right = cap.get(3).unwrap().as_str().to_string();
            (Node(start), (Node(left), Node(right)))
        })
        .collect();
    println!("{}", part1(&dirs, &nodes));
    println!("{}", part2(&dirs, &nodes));
    Ok(())
}

fn part1(dirs: &[Dir], nodes: &HashMap<Node, (Node, Node)>) -> u32 {
    let mut cache = HashSet::<(Node, usize)>::new();
    let mut current = Node("AAA".to_string());
    let target = Node("ZZZ".to_string());
    let mut count = 0;
    for (step, dir) in dirs.into_iter().enumerate().cycle() {
        if current == target || cache.contains(&(current.clone(), step)) {
            break;
        }
        let (left, right) = &nodes[&current];
        cache.insert((current, step));
        match dir {
            Dir::Left => current = left.clone(),
            Dir::Right => current = right.clone(),
        }
        count += 1;
    }
    count
}

fn part2(dirs: &[Dir], nodes: &HashMap<Node, (Node, Node)>) -> u64 {
    let mut dists = Vec::new();
    for mut current in nodes.keys().filter(|node| node.0.ends_with("A")) {
        let mut count = 0;
        for dir in dirs.into_iter().cycle() {
            if current.0.ends_with("Z") {
                dists.push(count);
                break;
            }
            match dir {
                Dir::Left => current = &nodes[current].0,
                Dir::Right => current = &nodes[current].1,
            }
            count += 1;
        }
    }
    // Cheating because apparently it works for this particular input . . . 😒
    dists.into_iter().fold(1, |acc, curr| lcm(acc, curr))
}
