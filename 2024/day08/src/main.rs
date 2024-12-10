use std::collections::{HashMap, HashSet};

use anyhow::Result;
use glam::IVec2;
use itertools::Itertools;

fn main() -> Result<()> {
    let content = std::fs::read_to_string("day08/input.txt")?;
    let lines = content.lines().collect::<Vec<_>>();
    let w = lines[0].len() as i32;
    let h = lines.len() as i32;

    let grid = lines
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                c.is_alphanumeric()
                    .then_some((IVec2::new(x as i32, y as i32), c))
            })
        })
        .fold(Grid::new(w, h), |mut grid, (pos, antenna)| {
            grid.antennas.entry(antenna).or_default().insert(pos);
            grid
        });

    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
    Ok(())
}

struct Grid {
    pub size: IVec2,
    pub antennas: HashMap<char, HashSet<IVec2>>,
}

impl Grid {
    pub fn new(w: i32, h: i32) -> Self {
        Self {
            size: IVec2::new(w, h),
            antennas: Default::default(),
        }
    }

    pub fn contains(&self, pos: &IVec2) -> bool {
        pos.cmpge(IVec2::ZERO).all() && pos.cmplt(self.size).all()
    }
}

fn part1(grid: &Grid) -> i32 {
    grid.antennas
        .iter()
        .flat_map(|(_, locations)| {
            locations.iter().combinations(2).flat_map(|tuple| {
                let (a, b) = (tuple[0], tuple[1]);
                vec![a + (a - b), b + (b - a)].into_iter()
            })
        })
        .filter(|pos| grid.contains(pos))
        .collect::<HashSet<_>>()
        .len() as i32
}

fn part2(grid: &Grid) -> i32 {
    grid.antennas
        .iter()
        .flat_map(|(_, locations)| {
            locations.iter().combinations(2).flat_map(|tuple| {
                let (a, b) = (tuple[0], tuple[1]);
                let first = (0..)
                    .map(move |k| a + k * (a - b))
                    .take_while(|pos| grid.contains(pos));
                let second = (0..)
                    .map(move |k| b + k * (b - a))
                    .take_while(|pos| grid.contains(pos));
                first.chain(second)
            })
        })
        .filter(|pos| grid.contains(pos))
        .collect::<HashSet<_>>()
        .len() as i32
}
