#![feature(array_zip)]

use anyhow::Result;
use std::collections::HashSet;

type Pos = [i32; 3];

const DIR: [Pos; 6] = [
    [-1, 0, 0],
    [1, 0, 0],
    [0, -1, 0],
    [0, 1, 0],
    [0, 0, -1],
    [0, 0, 1],
];

#[derive(Debug)]
struct AABB {
    pub min: Pos,
    pub max: Pos,
}

fn main() -> Result<()> {
    let content = std::fs::read_to_string("day18/input.txt")?;
    let blocks: HashSet<Pos> = content
        .lines()
        .map(|line| {
            let mut it = line.split(",").map(|coord| coord.parse().unwrap());
            [it.next().unwrap(), it.next().unwrap(), it.next().unwrap()]
        })
        .collect();
    println!("{}", part1(&blocks));
    println!("{}", part2(&blocks));
    Ok(())
}

fn part1(blocks: &HashSet<Pos>) -> usize {
    exposed(blocks)
}

fn part2(blocks: &HashSet<Pos>) -> usize {
    let bounds = bounds(blocks).extend(1);
    let complementary = flood_fill(blocks, &bounds);
    exposed(&complementary) - bounds.surface()
}

fn exposed(blocks: &HashSet<Pos>) -> usize {
    blocks
        .iter()
        .map(|&[x, y, z]| {
            6 - DIR
                .iter()
                .filter(|&&[dx, dy, dz]| blocks.contains(&[x + dx, y + dy, z + dz]))
                .count()
        })
        .sum()
}

fn bounds(blocks: &HashSet<Pos>) -> AABB {
    let [min, max] = blocks.iter().fold(
        [[i32::MAX; 3], [i32::MIN; 3]],
        |[botleft, topright], pos| [min(botleft, pos.clone()), max(topright, pos.clone())],
    );
    AABB { min, max }
}

fn flood_fill(blocks: &HashSet<Pos>, bounds: &AABB) -> HashSet<Pos> {
    let mut res = HashSet::new();
    let mut queue: HashSet<_> = HashSet::from_iter([bounds.min]);
    while !queue.is_empty() {
        res.extend(queue.iter());
        let neighbors: Vec<_> = queue
            .iter()
            .flat_map(|&[x, y, z]| {
                DIR.iter()
                    .map(move |&[dx, dy, dz]| [x + dx, y + dy, z + dz])
                    .filter(|pos| {
                        !blocks.contains(pos) && bounds.contains(pos) && !res.contains(pos)
                    })
            })
            .collect();
        queue.clear();
        queue.extend(neighbors);
    }
    res
}

#[inline]
fn min(a: Pos, b: Pos) -> Pos {
    a.zip(b).map(|(a, b)| a.min(b))
}

#[inline]
fn max(a: Pos, b: Pos) -> Pos {
    a.zip(b).map(|(a, b)| a.max(b))
}

impl AABB {
    pub fn extend(&self, amount: i32) -> Self {
        Self {
            min: self.min.map(|a| a - amount),
            max: self.max.map(|a| a + amount),
        }
    }

    pub fn contains(&self, [x, y, z]: &Pos) -> bool {
        let [minx, miny, minz] = self.min;
        let [maxx, maxy, maxz] = self.max;
        (minx..=maxx).contains(x) && (miny..=maxy).contains(y) && (minz..=maxz).contains(z)
    }

    pub fn surface(&self) -> usize {
        let [w, h, d] = self.min.zip(self.max).map(|(low, high)| high - low + 1);
        2 * (w * h + w * d + h * d) as usize
    }
}
