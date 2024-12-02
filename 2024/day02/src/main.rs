use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let content = fs::read_to_string("day02/example.txt");
    println!("Hello, world!");
    Ok(())
}
