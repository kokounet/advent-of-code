use anyhow::Error;
use itertools::iproduct;
use std::fs;

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("day08/input.txt")?;
    let grid: Vec<Vec<_>> = content
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let dim = (grid.first().unwrap().len(), grid.len());
    println!("{}", part1(&grid, dim));
    println!("{}", part2(&grid, dim));
    Ok(())
}

fn part1(grid: &Vec<Vec<u32>>, (w, h): (usize, usize)) -> u32 {
    assert!(w > 2 && h > 2);
    let ext = (w * h - (w - 2) * (h - 2)) as u32;
    let visible = iproduct!(1..h - 1, 1..w - 1)
        .filter(|&tree| is_visible(grid, tree, (w, h)))
        .count() as u32;

    ext + visible
}

fn is_visible(grid: &Vec<Vec<u32>>, (y, x): (usize, usize), (w, h): (usize, usize)) -> bool {
    let cmp = |i: usize, j: usize| grid[y][x] > grid[i][j];
    (0..y).all(|i| cmp(i,x))             // TOP
        || (y + 1..h).all(|i| cmp(i,x))  // BOTTOM
        || (0..x).all(|j| cmp(y, j))     // LEFT
        || (x + 1..w).all(|j| cmp(y, j)) // RIGHT
}

fn part2(grid: &Vec<Vec<u32>>, (w, h): (usize, usize)) -> u32 {
    assert!(w > 2 && h > 2);
    iproduct!(1..h - 1, 1..w - 1)
        .map(|tree| score(grid, tree, (w, h)))
        .max()
        .unwrap_or(0)
}

fn score(grid: &Vec<Vec<u32>>, (y, x): (usize, usize), (w, h): (usize, usize)) -> u32 {
    let cmp = |i: usize, j: usize| grid[y][x] > grid[i][j];
    let top = y - (0..y).rev().skip_while(|&i| cmp(i, x)).skip(1).count();
    let bot = h - (y + 1) - (y + 1..h).skip_while(|&i| cmp(i, x)).skip(1).count();
    let left = x - (0..x).rev().skip_while(|&j| cmp(y, j)).skip(1).count();
    let right = w - (x + 1) - (x + 1..w).skip_while(|&j| cmp(y, j)).skip(1).count();
    let score = top * bot * left * right;
    score as u32
}
