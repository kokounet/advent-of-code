use anyhow::Result;
use pathfinding::prelude::{dijkstra, dijkstra_all};
use std::collections::HashMap;
use std::fs;

type Pos = (usize, usize);
type Cost = usize;

struct Map {
    pub grid: Vec<Vec<usize>>,
}

fn main() -> Result<()> {
    let content = fs::read_to_string("day12/input.txt")?;
    let (start, end, map) = parse(&content);
    println!("{}", part1(&map, &start, &end));
    println!("{}", part2(&map, &start, &end));
    Ok(())
}

fn part1(map: &Map, start: &Pos, end: &Pos) -> Cost {
    map.shortest_path(start, end).unwrap().1
}

fn part2(map: &Map, _start: &Pos, end: &Pos) -> Cost {
    map.all_paths_to(end)
        .into_iter()
        .filter_map(|(pos, cost)| if map.at(&pos) == 1 { Some(cost) } else { None })
        .min()
        .unwrap()
}

fn parse(content: &str) -> (Pos, Pos, Map) {
    let heights: HashMap<_, _> = ('a'..='z')
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .chain([('S', 1), ('E', 26)]) // add elevation for start and end
        .collect();
    let (start, end, grid) = content.lines().enumerate().fold(
        ((0, 0), (0, 0), vec![]),
        |(mut start, mut end, mut grid), (i, line)| {
            if let Some(j) = line.find('S') {
                start = (i, j);
            }
            if let Some(j) = line.find('E') {
                end = (i, j);
            }
            grid.push(line.chars().map(|c| heights[&c]).collect::<Vec<_>>());
            (start, end, grid)
        },
    );
    (start, end, Map { grid })
}

impl Map {
    pub fn at(&self, &(l, c): &Pos) -> usize {
        self.grid[l][c]
    }

    pub fn neighbors(&self, &(l, c): &Pos) -> impl Iterator<Item = (Pos, Cost)> {
        let (h, w) = (self.grid.len() as i32, self.grid[0].len() as i32);
        let (il, ic) = (l as i32, c as i32);
        [(il - 1, ic), (il + 1, ic), (il, ic - 1), (il, ic + 1)]
            .into_iter()
            .filter(move |(il, ic)| (0..h).contains(il) && (0..w).contains(ic))
            .map(|(il, ic)| ((il as usize, ic as usize), 1))
    }

    pub fn shortest_path(&self, from: &Pos, to: &Pos) -> Option<(Vec<Pos>, Cost)> {
        dijkstra(
            from,
            |pos| {
                let height = self.at(pos) + 1;
                self.neighbors(pos)
                    .filter(move |(neighbor, _)| self.at(neighbor) <= height)
            },
            |pos| pos == to,
        )
    }

    pub fn all_paths_to(&self, to: &Pos) -> Vec<(Pos, Cost)> {
        dijkstra_all(to, |pos| {
            let height = self.at(pos) - 1;
            self.neighbors(pos)
                .filter(move |(neighbor, _)| self.at(neighbor) >= height)
        })
        .into_iter()
        .map(|(k, v)| (k, v.1))
        .collect()
    }
}
