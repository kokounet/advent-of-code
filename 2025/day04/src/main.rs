use std::{collections::BTreeSet, fs, mem::needs_drop};

use anyhow::Result;
use common::time;

fn main() -> Result<()> {
    let content = fs::read_to_string("day04/input.txt")?;
    let grid = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c == '@')
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();
    println!("{}", time!(part1(&grid)));
    println!("{}", time!(part2(grid)));
    Ok(())
}

fn part1(grid: &[Box<[bool]>]) -> i32 {
    let size = (grid.len() as isize, grid[0].len() as isize);
    grid.iter()
        .enumerate()
        .flat_map(|(l, line)| {
            line.iter().enumerate().filter_map(move |(c, &roll)| {
                let pos = (l as isize, c as isize);
                (roll && neighbors(grid, &pos, &size) < 4).then_some(pos)
            })
        })
        .count() as i32
}

fn part2(mut grid: Box<[Box<[bool]>]>) -> i32 {
    fn step(grid: &mut [Box<[bool]>]) -> i32 {
        let size = (grid.len() as isize, grid[0].len() as isize);
        let mut removable = BTreeSet::new();
        for (l, line) in grid.iter().enumerate() {
            for (c, &roll) in line.iter().enumerate() {
                let pos = (l as isize, c as isize);
                if roll && neighbors(grid, &pos, &size) < 4 {
                    removable.insert(pos);
                }
            }
        }
        let num = removable.len();
        for (l, c) in removable {
            grid[l as usize][c as usize] = false;
        }
        num as i32
    };
    std::iter::repeat_with(move || step(&mut grid))
        .take_while(|&num| num != 0)
        .sum()
}

fn neighbors(grid: &[Box<[bool]>], &(y, x): &(isize, isize), &(sy, sx): &(isize, isize)) -> i32 {
    const OFFSETS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    OFFSETS
        .iter()
        .filter(|(dy, dx)| {
            let nx = x + dx;
            let ny = y + dy;
            (0..sx).contains(&nx) && (0..sy).contains(&ny) && grid[ny as usize][nx as usize]
        })
        .count() as i32
}
