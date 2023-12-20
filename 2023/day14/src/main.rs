use anyhow::Error;
use grid::Grid;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("day14/input.txt")?;
    let grid = Grid::from(
        content
            .lines()
            .map(|line| line.chars().map(Cell::from).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );
    println!("{}", part1(grid.clone()));
    println!("{}", part2(grid.clone()));
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Rock,
    Cube,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    N,
    E,
    S,
    W,
}

fn part1(mut grid: Grid<Cell>) -> usize {
    tilt(&mut grid, Dir::N);
    score(&grid)
}

fn part2(mut grid: Grid<Cell>) -> usize {
    let mut cache = HashMap::new();
    for i in 0..1_000_000_000 {
        let mut hasher = DefaultHasher::new();

        cycle(&mut grid);
        let hash = {
            grid.hash(&mut hasher);
            hasher.finish()
        };

        if let Some((idx, _)) = cache.get(&hash) {
            let cycle_len = i - idx;
            let map: BTreeMap<_, _> = cache.values().cloned().collect();
            let offset = idx;
            let finish_idx = ((1_000_000_000 - i - 1) % cycle_len) + offset;
            return map[&finish_idx];
        }
        let curr_score = score(&grid);
        cache.insert(hash, (i, curr_score));
    }
    0
}

fn tilt(grid: &mut Grid<Cell>, dir: Dir) {
    let (row, col) = grid.size();
    match dir {
        Dir::N => {
            tilt_helper(grid, 0..col, 0..row, 0, 1, false);
        }
        Dir::S => {
            tilt_helper(grid, 0..col, (0..row).rev(), row - 1, -1, false);
        }
        Dir::W => {
            tilt_helper(grid, 0..row, 0..col, 0, 1, true);
        }
        Dir::E => {
            tilt_helper(grid, 0..row, (0..col).rev(), col - 1, -1, true);
        }
    }
}

fn cycle(grid: &mut Grid<Cell>) {
    tilt(grid, Dir::N);
    tilt(grid, Dir::W);
    tilt(grid, Dir::S);
    tilt(grid, Dir::E);
}

// Dark magic to avoid repeating myself
// major_iter:  Direction orthogonal to the tilt ()
// minor_iter:  column or line where the gravity should be applied
//              form the bottom to the top
// start:       the position of the ground at the begining of each iteration
// dir:         increment you add to the ground to make it move up (+1 or -1)
// flip:        used to flip the indices before accessing the grid (false for
//              N or S, true for W or E)
fn tilt_helper<It1, It2>(
    grid: &mut Grid<Cell>,
    major_iter: It1,
    minor_iter: It2,
    start: usize,
    dir: isize,
    flip: bool,
) where
    It1: Iterator<Item = usize>,
    It2: Iterator<Item = usize> + Clone,
{
    for maj in major_iter {
        let mut ground = start;
        for min in minor_iter.clone() {
            let idx = if flip { (maj, min) } else { (min, maj) };
            let ground_idx = if flip { (maj, ground) } else { (ground, maj) };
            match grid[idx] {
                Cell::Cube => ground = (min as isize + dir) as usize,
                Cell::Rock => {
                    if ground != min {
                        swap(grid, idx, ground_idx);
                    }
                    ground = (ground as isize + dir) as usize;
                }
                _ => {}
            }
        }
    }
}

fn score(grid: &Grid<Cell>) -> usize {
    let mut total = 0;
    let (_, col) = grid.size();
    for (i, row) in grid.iter_rows().enumerate() {
        total += row
            .filter_map(|cell| matches!(cell, Cell::Rock).then_some(col - i))
            .sum::<usize>();
    }
    total
}

fn swap(grid: &mut Grid<Cell>, a: (usize, usize), b: (usize, usize)) {
    //println!("swap: {a:?}<->{b:?}");
    let tmp = grid[a];
    grid[a] = grid[b];
    grid[b] = tmp;
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Cube,
            'O' => Self::Rock,
            _ => Self::Empty,
        }
    }
}
