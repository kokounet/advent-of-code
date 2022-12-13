use anyhow::Result;
use itertools::{EitherOrBoth::*, FoldWhile::*, Itertools};
use nom::{
    branch::alt,
    character::complete::{char, digit1},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};
use std::cmp::Ordering;
use std::fs;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

fn main() -> Result<()> {
    let content = fs::read_to_string("day13/input.txt")?;
    let packets: Vec<_> = content
        .lines()
        .filter_map(|line| parse(line).ok().map(|c| c.1))
        .collect();
    println!("{}", part1(&packets));
    println!("{}", part2(packets));
    Ok(())
}

fn part1(packets: &[Packet]) -> usize {
    packets
        .chunks_exact(2)
        .enumerate()
        .filter_map(|(i, chunk)| {
            let left = &chunk[0];
            let right = &chunk[1];
            if left < right {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

fn part2(mut packets: Vec<Packet>) -> usize {
    let two = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let six = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);
    packets.sort();
    let id2 = packets.binary_search(&two).unwrap_err() + 1;
    let id6 = packets.binary_search(&six).unwrap_err() + 2;
    id2 * id6
}

fn parse(list: &str) -> IResult<&str, Packet> {
    let (rest, list) = delimited(
        char('['),
        separated_list0(char(','), alt((parse, parse_int))),
        char(']'),
    )(list)?;
    Ok((rest, Packet::List(list)))
}

fn parse_int(number: &str) -> IResult<&str, Packet> {
    let (rest, number) = digit1(number)?;
    Ok((rest, Packet::Int(number.parse().unwrap())))
}

impl Packet {
    fn to_list(&self) -> Vec<Self> {
        match self {
            &Self::Int(v) => vec![Self::Int(v)],
            Self::List(list) => list.clone(),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let (Self::Int(u), Self::Int(v)) = (self, other) {
            return u.cmp(v);
        }
        let (left, right) = (self.to_list(), other.to_list());
        left.iter()
            .zip_longest(right.iter())
            .fold_while(Ordering::Equal, |cmp, curr| {
                if !cmp.is_eq() {
                    return Done(cmp);
                }
                match curr {
                    Left(_) => Done(Ordering::Greater),
                    Right(_) => Done(Ordering::Less),
                    Both(l, r) => Continue(l.cmp(r)),
                }
            })
            .into_inner()
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
