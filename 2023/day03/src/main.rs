use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let content = fs::read_to_string("day03/example.txt")?;
    let lines: Vec<_> = content.lines().collect();
    Ok(())
}
