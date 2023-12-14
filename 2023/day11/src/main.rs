use anyhow::Error;
use std::collections::BTreeSet;
use std::fs;

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("day11/input.txt")?;
    let galaxies: BTreeSet<_> = content
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, c)| match c {
                    '.' => None,
                    '#' => Some((row, col)),
                    _ => unreachable!(),
                })
        })
        .collect();
    println!("{}", part1(&galaxies));
    println!("{}", part2(&galaxies));
    Ok(())
}

fn part1(galaxies: &BTreeSet<(usize, usize)>) -> usize {
    let expanded = expand(galaxies, 2);
    let mut sum = 0;
    for (row1, col1) in expanded.iter() {
        for (row2, col2) in expanded.range((*row1, *col1)..) {
            if (row1 == row2) && (col1 == col2) {
                continue;
            }
            let dist = row2.abs_diff(*row1) + col2.abs_diff(*col1);
            sum += dist;
        }
    }
    sum
}

fn part2(galaxies: &BTreeSet<(usize, usize)>) -> usize {
    let expanded = expand(galaxies, 1_000_000);
    let mut sum = 0;
    for (row1, col1) in expanded.iter() {
        for (row2, col2) in expanded.range((*row1, *col1)..) {
            if (row1 == row2) && (col1 == col2) {
                continue;
            }
            let dist = row2.abs_diff(*row1) + col2.abs_diff(*col1);
            sum += dist;
        }
    }
    sum
}

fn expand(galaxies: &BTreeSet<(usize, usize)>, factor: usize) -> BTreeSet<(usize, usize)> {
    let (rows, cols): (BTreeSet<_>, BTreeSet<_>) = galaxies.iter().cloned().unzip();
    let all_rows: BTreeSet<_> = (0..=*rows.last().unwrap()).collect();
    let all_cols: BTreeSet<_> = (0..=*cols.last().unwrap()).collect();
    let empty_rows: BTreeSet<_> = all_rows.difference(&rows).cloned().collect();
    let empty_cols: BTreeSet<_> = all_cols.difference(&cols).cloned().collect();
    galaxies
        .iter()
        .map(|(row, col)| {
            let rexpand = empty_rows.range(..row).count();
            let cexpand = empty_cols.range(..col).count();
            (row + rexpand * (factor - 1), col + cexpand * (factor - 1))
        })
        .collect()
}
