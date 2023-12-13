use anyhow::Error;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Cell {
    Empty,
    Galaxy,
}

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("day11/example.txt")?;
    let map: Vec<Vec<_>> = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '#' => Cell::Galaxy,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    let expanded = expand(&map);
    display_map(&map);
    display_map(&expanded);
    Ok(())
}

fn expand(map: &[Vec<Cell>]) -> Vec<Vec<Cell>> {
    let mut new = map.to_vec();
    let rows = map.len();
    let cols = map[0].len();
    let empty_rows: Vec<_> = map
        .iter()
        .enumerate()
        .filter_map(|(i, row)| {
            row.iter()
                .all(|cell| matches!(cell, Cell::Empty))
                .then_some(i)
        })
        .collect();
    let empty_cols: Vec<_> = (0..cols)
        .filter_map(|col| {
            map.iter()
                .all(|row| matches!(row[col], Cell::Empty))
                .then_some(col)
        })
        .collect();
    for row in new.iter_mut() {
        for empty_col in empty_cols.iter().rev() {
            row.insert(*empty_col, Cell::Empty);
        }
    }
    let new_cols = new[0].len();
    let empty = vec![Cell::Empty; new_cols];
    for empty_row in empty_rows.iter().rev() {
        new.insert(*empty_row, empty.clone());
    }
    let new_rows = new.len();
    println!("({rows}, {cols})->({new_rows}, {new_cols})");
    new
}

fn display_map(map: &[Vec<Cell>]) {
    for row in map.iter() {
        for cell in row {
            print!(
                "{}",
                match cell {
                    Cell::Empty => '.',
                    Cell::Galaxy => '#',
                }
            );
        }
        println!("");
    }
}
