use anyhow::Error;
use std::collections::HashSet;
use std::fs;
use std::hash::Hash;

fn main() -> Result<(), Error> {
    let content: Vec<_> = fs::read_to_string("day06/input.txt")?
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();
    println!("{}", part1(&content));
    println!("{}", part2(&content));
    Ok(())
}

fn part1(content: &[char]) -> u32 {
    marker(content, 4)
}

fn part2(content: &[char]) -> u32 {
    marker(content, 14)
}

fn marker(content: &[char], len: usize) -> u32 {
    content
        .windows(len)
        .enumerate()
        .find(|(_, win)| all_different(win))
        .map(|(i, _)| i + len)
        .unwrap() as u32
}

fn all_different<T: Eq + Hash>(collection: &[T]) -> bool {
    let mut set = HashSet::with_capacity(collection.len());
    for e in collection.iter() {
        if set.contains(e) {
            return false;
        }
        set.insert(e);
    }
    true
}
