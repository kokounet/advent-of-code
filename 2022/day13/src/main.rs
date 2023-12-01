mod packet;

use anyhow::Result;
use nom::{
    branch::alt,
    character::complete::{char, digit1},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};
use std::fs;

use crate::packet::Packet;

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
            if chunk[0] < chunk[1] {
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
