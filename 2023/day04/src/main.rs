use anyhow::{anyhow, Error};
use std::{collections::BTreeSet, fs};

#[derive(Debug, Default)]
struct Card {
    pub win: BTreeSet<u32>,
    pub actual: BTreeSet<u32>,
}

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("day04/input.txt")?;
    let cards: Vec<Card> = content
        .lines()
        .filter_map(|line| line.try_into().ok())
        .collect();
    println!("{}", part1(&cards));
    println!("{}", part2(&cards));
    Ok(())
}

fn part1(cards: &[Card]) -> u32 {
    cards
        .into_iter()
        .map(|card| {
            let wins = card.win.intersection(&card.actual).count();
            if wins == 0 {
                0
            } else {
                1 << (wins - 1)
            }
        })
        .sum()
}

fn part2(cards: &[Card]) -> u32 {
    // we start with at least one instance of each card
    let mut instances = vec![1; cards.len()];
    for (i, card) in cards.into_iter().enumerate() {
        let copies = instances[i];
        let wins = card.win.intersection(&card.actual).count();
        for j in (i + 1)..(i + 1 + wins) {
            instances[j] += copies;
        }
    }
    instances.iter().sum()
}

impl TryFrom<&str> for Card {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split = value.trim().split(":");
        let numbers = split.last().unwrap().trim().split("|");
        let mut sets = numbers.map(|set| {
            set.trim()
                .split_whitespace()
                .filter_map(|num| num.parse::<u32>().ok())
                .collect::<BTreeSet<_>>()
        });
        Ok(Card {
            win: sets.next().ok_or(anyhow!(""))?,
            actual: sets.next().ok_or(anyhow!(""))?,
        })
    }
}
