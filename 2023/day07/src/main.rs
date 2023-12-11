use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::BTreeMap;
use std::fs;

use anyhow::{anyhow, Error};

#[derive(Debug)]
struct Bid {
    pub hand: Hand,
    pub bet: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand(pub Vec<Card>);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    J,
    Num(u8),
    T,
    Q,
    K,
    A,
}

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("day07/input.txt")?;
    let mut bids = content
        .lines()
        .map(|line| line.try_into())
        .collect::<Result<Vec<Bid>, _>>()?;
    bids.sort_by(|left, right| left.hand.cmp(&right.hand));
    println!("{}", part1(&bids));
    println!("{}", part2(&bids));
    Ok(())
}

fn part1(bids: &[Bid]) -> u32 {
    bids.into_iter()
        .enumerate()
        .map(|(rank, bid)| (rank as u32 + 1) * bid.bet)
        .sum()
}

fn part2(bids: &[Bid]) -> u32 {
    0
}

/// ORDERING
impl Hand {
    fn top(&self) -> Vec<u8> {
        let mut counter = BTreeMap::new();
        for card in self.0.iter() {
            counter
                .entry(*card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let mut top: Vec<_> = counter.into_values().collect();
        top.sort();
        top
    }

    fn kind(&self) -> HandKind {
        use HandKind::*;
        let mut top = self.top();
        let first = top.pop().unwrap();
        let second = top.pop().unwrap_or_default();
        match (first, second) {
            (5, 0) => FiveOfAKind,
            (4, 1) => FourOfAKind,
            (3, 2) => FullHouse,
            (3, 1) => ThreeOfAKind,
            (2, 2) => TwoPairs,
            (2, 1) => OnePair,
            (1, _) => HighCard,
            _ => unreachable!(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.kind()
            .cmp(&other.kind())
            .then_with(|| self.0.cmp(&other.0))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Card {
    pub fn all() -> impl Iterator<Item = Card> {
        use Card::*;
        (2..=9).map(|num| Num(num)).chain([T, Q, K, A])
    }
}

/// PARSING
impl TryFrom<&str> for Bid {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split_whitespace();
        let hand = split.next().unwrap().try_into()?;
        let bet = split.next().unwrap().parse::<u32>()?;
        Ok(Bid { hand, bet })
    }
}

impl TryFrom<&str> for Hand {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let cards = value
            .chars()
            .map(|c| c.try_into())
            .collect::<Result<Vec<Card>, _>>()?;
        Ok(Hand(cards))
    }
}

impl TryFrom<char> for Card {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::T),
            num @ '2'..='9' => Ok(Card::Num(num.to_digit(10).unwrap() as u8)),
            _ => Err(anyhow!("Parsing Card Error")),
        }
    }
}
