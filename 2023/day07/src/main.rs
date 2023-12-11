use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::BTreeMap;
use std::fs;

use anyhow::{anyhow, Error};

#[derive(Debug, Clone)]
struct Bid {
    pub hand: Hand,
    pub bet: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand(pub Vec<Card>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    Num(u8),
    T,
    J,
    Q,
    K,
    A,
}

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("day07/input.txt")?;
    let bids = content
        .lines()
        .map(|line| line.try_into())
        .collect::<Result<Vec<Bid>, _>>()?;
    println!("{}", part1(bids.clone()));
    println!("{}", part2(bids.clone()));
    Ok(())
}

fn part1(mut bids: Vec<Bid>) -> u32 {
    bids.sort_by(|a, b| a.hand.cmp(&b.hand));
    bids.into_iter()
        .enumerate()
        .map(|(rank, bid)| (rank as u32 + 1) * bid.bet)
        .sum()
}

fn part2(mut bids: Vec<Bid>) -> u32 {
    bids.sort_by(|a, b| a.hand.cmp2(&b.hand));
    bids.into_iter()
        .enumerate()
        .map(|(rank, bid)| (rank as u32 + 1) * bid.bet)
        .sum()
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

    fn top2(&self) -> Vec<u8> {
        // same as top but filter out the jokers
        let mut counter = BTreeMap::new();
        for card in self.0.iter().filter(|card| !matches!(card, Card::J)) {
            counter
                .entry(*card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let mut top: Vec<_> = counter.into_values().collect();
        top.sort();
        top
    }

    fn kind2(&self) -> HandKind {
        use HandKind::*;
        let jokers = self
            .0
            .iter()
            .filter(|&card| matches!(card, Card::J))
            .count() as u8;
        if jokers == 0 {
            return self.kind();
        }
        let mut top = self.top2();
        let first = top.pop().unwrap_or_default() + jokers;
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

    fn cmp2(&self, other: &Self) -> Ordering {
        self.kind2().cmp(&other.kind2()).then_with(|| {
            self.0
                .iter()
                .map(Card2::from)
                .cmp(other.0.iter().map(Card2::from))
        })
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

/// Card hack
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card2 {
    J,
    Num(u8),
    T,
    Q,
    K,
    A,
}

impl From<&Card> for Card2 {
    fn from(value: &Card) -> Self {
        match &value {
            Card::J => Card2::J,
            Card::Num(num) => Card2::Num(*num),
            Card::T => Card2::T,
            Card::Q => Card2::Q,
            Card::K => Card2::K,
            Card::A => Card2::A,
        }
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
