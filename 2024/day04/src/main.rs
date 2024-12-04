use std::fs;

use anyhow::Result;
use itertools::iproduct;

fn main() -> Result<()> {
    let content = fs::read_to_string("day04/input.txt")?;
    let lines: Vec<Vec<_>> = content.lines().map(|line| line.chars().collect()).collect();
    let grid = Grid {
        width: lines[0].len() as isize,
        height: lines.len() as isize,
        data: lines
            .into_iter()
            .flat_map(|line| line.into_iter())
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    };
    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
    Ok(())
}

struct Grid {
    pub width: isize,
    pub height: isize,
    data: Box<[char]>,
}

impl Grid {
    pub fn get(&self, x: isize, y: isize) -> Option<char> {
        if !(0..self.width).contains(&x) || !(0..self.height).contains(&y) {
            return None;
        }
        let i = (y * self.width + x) as usize;
        Some(self.data[i])
    }

    pub fn word(
        &self,
        start: (isize, isize),
        deltas: impl Iterator<Item = (isize, isize)>,
    ) -> String {
        let (x, y) = start;
        deltas
            .filter_map(|(dx, dy)| self.get(x + dx, y + dy))
            .collect()
    }
}

fn part1(grid: &Grid) -> i32 {
    let dirs = [
        [(0, 0), (-1, -1), (-2, -2), (-3, -3)], // up left
        [(0, 0), (-1, 0), (-2, 0), (-3, 0)],    // left
        [(0, 0), (-1, 1), (-2, 2), (-3, 3)],    // down left
        [(0, 0), (0, -1), (0, -2), (0, -3)],    // up
        [(0, 0), (0, 1), (0, 2), (0, 3)],       // down
        [(0, 0), (1, -1), (2, -2), (3, -3)],    // up right
        [(0, 0), (1, 0), (2, 0), (3, 0)],       // right
        [(0, 0), (1, 1), (2, 2), (3, 3)],       // down right
    ];

    iproduct!(0..grid.width, 0..grid.height)
        .map(|start| {
            dirs.iter()
                .filter(move |dir| grid.word(start, dir.iter().cloned()) == "XMAS")
                .count() as i32
        })
        .sum()
}

fn part2(grid: &Grid) -> i32 {
    let dirs = [
        [(-1, -1), (0, 0), (1, 1)],
        [(1, 1), (0, 0), (-1, -1)],
        [(-1, 1), (0, 0), (1, -1)],
        [(1, -1), (0, 0), (-1, 1)],
    ];

    iproduct!(0..grid.width, 0..grid.height)
        .filter(|&(x, y)| {
            if grid.get(x, y).unwrap() != 'A' {
                return false; // A is always at the center of the pattern
            }
            dirs.iter()
                .filter(move |dir| grid.word((x, y), dir.iter().cloned()) == "MAS")
                .count()
                >= 2
        })
        .count() as i32
}
