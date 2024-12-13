use std::{
    collections::{HashMap, HashSet},
    ops::Index,
};

use glam::IVec2;

fn main() -> anyhow::Result<()> {
    let content = std::fs::read_to_string("day10/input.txt")?;
    let lines = content.lines().collect::<Vec<_>>();
    let w = lines[0].len() as i32;
    let h = lines.len() as i32;
    let mut grid = Vec::with_capacity((w * h) as usize);
    grid.extend(
        content
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8)),
    );
    let map = HeightMap {
        size: IVec2::new(w, h),
        grid: grid.into_boxed_slice(),
    };

    println!("{}", part1(&map));
    println!("{}", part2(&map));
    Ok(())
}

struct HeightMap {
    pub size: IVec2,
    grid: Box<[u8]>,
}

impl HeightMap {
    fn get(&self, pos: IVec2) -> Option<u8> {
        self.contains(pos).then(|| self.grid[self.linearize(pos)])
    }

    fn contains(&self, pos: IVec2) -> bool {
        pos.cmpge(IVec2::ZERO).all() && pos.cmplt(self.size).all()
    }

    fn linearize(&self, pos: IVec2) -> usize {
        (pos.x + pos.y * self.size.x) as usize
    }

    fn delinearize(&self, i: usize) -> IVec2 {
        let i = i as i32;
        IVec2::new(i.rem_euclid(self.size.x), i.div_euclid(self.size.y))
    }

    fn iter(&self) -> impl Iterator<Item = (IVec2, &u8)> {
        self.grid
            .iter()
            .enumerate()
            .map(|(i, height)| (self.delinearize(i), height))
    }
}

impl Index<IVec2> for HeightMap {
    type Output = u8;

    fn index(&self, index: IVec2) -> &Self::Output {
        &self.grid[self.linearize(index)]
    }
}

fn trails_from(
    start: IVec2,
    map: &HeightMap,
    cache: &mut HashMap<IVec2, Vec<Vec<IVec2>>>,
) -> Vec<Vec<IVec2>> {
    if let Some(trails) = cache.get(&start) {
        return trails.clone();
    }
    let h = map[start];
    if h == 9 {
        return vec![vec![start]];
    }
    let mut trails = Vec::new();
    for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)].map(IVec2::from) {
        let neighbor = start + dir;
        let Some(neighbor_h) = map.get(neighbor) else {
            continue;
        };
        if neighbor_h != h + 1 {
            continue;
        }
        trails.extend(
            trails_from(neighbor, map, cache)
                .into_iter()
                .map(|mut trail| {
                    trail.push(start);
                    trail
                }),
        );
    }
    cache.insert(start, trails.clone());
    trails
}

fn part1(map: &HeightMap) -> i32 {
    map.iter()
        .filter(|(_, &h)| h == 0)
        .map(|(start, _)| {
            let mut cache = HashMap::new();
            let trails = trails_from(start, map, &mut cache);
            let score = trails
                .iter()
                .map(|trail| trail[0])
                .collect::<HashSet<_>>()
                .len() as i32;
            score
        })
        .sum()
}

fn part2(map: &HeightMap) -> i32 {
    map.iter()
        .filter(|(_, &h)| h == 0)
        .map(|(start, _)| {
            let mut cache = HashMap::new();
            let trails = trails_from(start, map, &mut cache);
            let score = trails.len() as i32;
            score
        })
        .sum()
}
