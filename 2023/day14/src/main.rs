use anyhow::Error;
use ndarray::prelude::*;
use std::fs;

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("day14/example.txt")?;

    println!("Hello, world!");
    Ok(())
}

enum Cell {
    Empty,
    Rock,
    Cube,
}
