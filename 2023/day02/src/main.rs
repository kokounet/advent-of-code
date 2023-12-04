use anyhow::Error;
use std::fs;

#[derive(Debug)]
struct Game {
    pub id: u32,
    pub rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("day02/input.txt")?;
    let games = content
        .lines()
        .map(|line| line.try_into())
        .collect::<Result<Vec<Game>, _>>()?;
    println!("{:?}", part1(&games));
    println!("{:?}", part2(&games));
    Ok(())
}

fn part1(games: &[Game]) -> u32 {
    let (r, g, b) = (12, 13, 14);
    games
        .into_iter()
        .filter_map(|game| {
            if game
                .rounds
                .iter()
                .all(|round| round.r <= r && round.g <= g && round.b <= b)
            {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

fn part2(games: &[Game]) -> u32 {
    games
        .into_iter()
        .map(|game| {
            let (r, g, b) = game.rounds.iter().fold((0, 0, 0), |(r, g, b), round| {
                (r.max(round.r), g.max(round.g), b.max(round.b))
            });
            r * g * b
        })
        .sum()
}

impl TryFrom<&str> for Game {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.trim().split(": ");
        let id: u32 = split.next().unwrap().trim_start_matches("Game ").parse()?;
        let rounds = split
            .next()
            .unwrap()
            .trim()
            .split("; ")
            .map(TryInto::try_into)
            .collect::<Result<Vec<Round>, _>>()?;
        Ok(Game { id, rounds })
    }
}

impl TryFrom<&str> for Round {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (mut r, mut g, mut b) = (0, 0, 0);
        for cube in value.split(", ") {
            let mut split = cube.split(" ");
            let number = split.next().unwrap().parse::<u32>().unwrap();
            let color = split.next().unwrap();
            match color {
                "red" => r = number,
                "green" => g = number,
                "blue" => b = number,
                _ => {}
            }
        }
        Ok(Round { r, g, b })
    }
}
