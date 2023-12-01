use Outcome::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    pub fn score(self) -> u32 {
        match self {
            Loss => 0,
            Draw => 3,
            Win => 6,
        }
    }
}

impl From<&str> for Outcome {
    fn from(letter: &str) -> Self {
        match letter {
            "X" => Loss,
            "Y" => Draw,
            "Z" => Win,
            _ => unreachable!(),
        }
    }
}
