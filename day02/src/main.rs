mod outcome;
mod shape;

use anyhow::Error;
use std::fs;

use crate::shape::Shape;

fn part1(rounds: &[(&str, &str)]) -> u32 {
    rounds
        .iter()
        .map(|&(opponent, shape)| {
            let opponent: Shape = opponent.into();
            let shape: Shape = shape.into();
            shape.score() + shape.outcome(&opponent).score()
        })
        .sum()
}

fn part2(rounds: &[(&str, &str)]) -> u32 {
    rounds
        .iter()
        .map(|&(opponent, outcome)| {
            let opponent = opponent.into();
            let outcome = outcome.into();
            Shape::for_outcome(opponent, outcome).score() + outcome.score()
        })
        .sum()
}

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("day02/input.txt")?;
    let rounds: Vec<(&str, &str)> = content
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let frag: Vec<_> = line.split_whitespace().take(2).collect();
            (frag[0], frag[1])
        })
        .collect();
    println!("{}", part1(&rounds));
    println!("{}", part2(&rounds));
    Ok(())
}
