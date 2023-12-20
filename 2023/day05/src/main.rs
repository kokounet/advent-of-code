use anyhow::{anyhow, Error};
use std::{fs, ops::Range};

#[derive(Debug)]
struct Mapping {
    pub source: Range<u64>,
    pub dest: u64,
}

#[derive(Debug, Default)]
struct Almanac {
    pub seeds: Vec<u64>,
    pub maps: Vec<Vec<Mapping>>,
}

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("day05/example.txt")?;
    let almanac: Almanac = content.as_str().try_into()?;
    println!("{}", part1(&almanac));
    Ok(())
}

fn part1(almanac: &Almanac) -> u64 {
    almanac
        .seeds
        .iter()
        .map(|&seed| almanac.seed_to_location(seed))
        .min()
        .unwrap()
}

impl Almanac {
    pub fn seed_to_location(&self, seed: u64) -> u64 {
        let mut current = seed;
        for map in self.maps.iter() {
            for mapping in map.iter() {
                if mapping.source.contains(&current) {
                    current -= mapping.source.start;
                    current += mapping.dest;
                    break;
                }
            }
        }
        current
    }
}

impl TryFrom<&str> for Almanac {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines = value.lines();
        let seeds = lines
            .next()
            .ok_or(anyhow!(""))?
            .trim_start_matches("seeds: ")
            .split_whitespace()
            .map(|num| num.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()?;
        lines.next();
        let mut maps = Vec::new();
        let mut mappings: Vec<Mapping> = Vec::new();
        for line in lines {
            if line.is_empty() {
                mappings.sort_by_key(|mapping| mapping.source.start);
                maps.push(mappings);
                mappings = Vec::new();
                continue;
            }
            if line.ends_with("map:") {
                continue;
            }
            let mut numbers = line.trim().split_whitespace().map(|num| num.parse::<u64>());
            let dest = numbers.next().unwrap()?;
            let source = numbers.next().unwrap()?;
            let length = numbers.next().unwrap()?;
            mappings.push(Mapping {
                source: source..(source + length),
                dest: dest,
            })
        }
        mappings.sort_by_key(|mapping| mapping.source.start);
        maps.push(mappings);
        Ok(Almanac { seeds, maps })
    }
}
