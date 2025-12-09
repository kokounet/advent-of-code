use std::collections::{BTreeSet, HashSet};

use anyhow::{anyhow, Result};
use common::time;
use itertools::Itertools;

fn main() -> Result<()> {
    let content = std::fs::read_to_string("day08/input.txt")?;
    let coords = content
        .lines()
        .map(|line| {
            let mut it = line.trim().split(",");
            let x = it.next().ok_or(anyhow!("x"))?.parse::<i64>()?;
            let y = it.next().ok_or(anyhow!("y"))?.parse::<i64>()?;
            let z = it.next().ok_or(anyhow!("z"))?.parse::<i64>()?;
            Ok([x, y, z])
        })
        .collect::<Result<BTreeSet<_>>>()?;
    println!("{}", time!(part1(&coords)));
    println!("{}", time!(part2(&coords)));
    Ok(())
}

fn dist2(a: &[i64; 3], b: &[i64; 3]) -> i64 {
    (a[0] - b[0]).pow(2) + (a[1] - b[1]).pow(2) + (a[2] - b[2]).pow(2)
}

fn dist_matrix(coords: &BTreeSet<[i64; 3]>) -> Vec<([i64; 3], [i64; 3])> {
    let mut mat: Vec<_> = coords
        .iter()
        .combinations(2)
        .map(|points| (*points[0], *points[1]))
        .collect();
    mat.sort_by(|(a1, a2), (b1, b2)| dist2(a1, a2).cmp(&dist2(b1, b2)));
    mat
}

fn part1(coords: &BTreeSet<[i64; 3]>) -> i64 {
    let dists = dist_matrix(coords);
    let mut circuits: Vec<HashSet<[i64; 3]>> = coords
        .iter()
        .map(|pos| HashSet::from_iter([*pos]))
        .collect();

    for (p1, p2) in &dists[..1000] {
        let i1 = circuits.iter().position(|set| set.contains(p1)).unwrap();
        let i2 = circuits.iter().position(|set| set.contains(p2)).unwrap();
        if i1 == i2 {
            continue;
        }
        let (min, max) = (i1.min(i2), i1.max(i2));
        let smax = circuits.remove(max);
        circuits[min].extend(smax);
    }
    let mut sizes: Vec<_> = circuits.into_iter().map(|set| set.len() as i64).collect();
    sizes.sort_unstable_by_key(|s| -s);
    sizes[0] * sizes[1] * sizes[2]
}

fn part2(coords: &BTreeSet<[i64; 3]>) -> i64 {
    let dists = dist_matrix(coords);
    let mut circuits: Vec<HashSet<[i64; 3]>> = coords
        .iter()
        .map(|pos| HashSet::from_iter([*pos]))
        .collect();

    for (p1, p2) in &dists {
        let i1 = circuits.iter().position(|set| set.contains(p1)).unwrap();
        let i2 = circuits.iter().position(|set| set.contains(p2)).unwrap();
        if i1 == i2 {
            continue;
        }
        let (min, max) = (i1.min(i2), i1.max(i2));
        let smax = circuits.remove(max);
        circuits[min].extend(smax);
        if circuits[min].len() == coords.len() {
            return p1[0] * p2[0];
        }
    }
    unreachable!()
}
