use crate::outcome::Outcome;

use Outcome::*;
use Shape::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn score(self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    pub fn outcome(&self, other: &Shape) -> Outcome {
        if self == other {
            return Draw;
        }
        match (self, other) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Win,
            _ => Loss,
        }
    }

    pub fn for_outcome(opponent: Shape, outcome: Outcome) -> Self {
        [Rock, Paper, Scissors]
            .into_iter()
            .find(|shape| shape.outcome(&opponent) == outcome)
            .unwrap()
    }
}

impl From<&str> for Shape {
    fn from(letter: &str) -> Self {
        match letter {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => unreachable!(),
        }
    }
}
