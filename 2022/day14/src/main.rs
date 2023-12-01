use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::{collections::HashSet, fs, ops::RangeInclusive};

type Grid = HashSet<(i32, i32)>;

#[derive(Debug)]
enum Path {
    Horizontal(i32, RangeInclusive<i32>),
    Vertical(i32, RangeInclusive<i32>),
}

fn main() -> Result<()> {
    let content = fs::read_to_string("day14/input.txt")?;
    let paths = content
        .lines()
        .map(parse)
        .map(|r| r.unwrap().1)
        .map(|points| {
            points
                .windows(2)
                .map(|line| match line {
                    &[(x1, y1), (x2, y2)] if x1 == x2 => {
                        let (ymin, ymax) = (y1.min(y2), y1.max(y2));
                        Path::Vertical(x1, ymin..=ymax)
                    }
                    &[(x1, y1), (x2, y2)] if y1 == y2 => {
                        let (xmin, xmax) = (x1.min(x2), x1.max(x2));
                        Path::Horizontal(y1, xmin..=xmax)
                    }
                    [..] => unreachable!(),
                })
                .collect()
        });
    let grid = Grid::from_iter(paths.flat_map(|paths: Vec<_>| {
        paths.into_iter().flat_map(|path| match path {
            Path::Horizontal(y, range) => range.map(|x| (x, y)).collect::<Vec<_>>(),
            Path::Vertical(x, range) => range.map(|y| (x, y)).collect::<Vec<_>>(),
        })
    }));
    println!("{}", part1(grid.clone()));
    println!("{}", part2(grid));
    Ok(())
}

fn part1(mut grid: Grid) -> usize {
    let [_, (_, ymax)] = bounds(&grid);
    std::iter::repeat(simulate1)
        .take_while(move |sim| sim(&mut grid, ymax))
        .count()
}

fn part2(mut grid: Grid) -> usize {
    let [_, (_, ymax)] = bounds(&grid);
    std::iter::repeat(simulate2)
        .take_while(move |sim| sim(&mut grid, ymax + 2))
        .count()
}

fn simulate1(grid: &mut Grid, ymax: i32) -> bool {
    let mut point = (500, 0);
    while let Some(new) = step(grid, &point) {
        if new.1 > ymax {
            return false;
        }
        point = new;
    }
    grid.insert(point)
}

fn simulate2(grid: &mut Grid, ymax: i32) -> bool {
    let mut point = (500, 0);
    while let Some(new) = step(grid, &point) {
        if new.1 >= ymax {
            break;
        }
        point = new;
    }
    grid.insert(point)
}

fn step(grid: &Grid, &(x, y): &(i32, i32)) -> Option<(i32, i32)> {
    let y = y + 1;
    if !grid.contains(&(x, y)) {
        Some((x, y))
    } else if !grid.contains(&(x - 1, y)) {
        Some((x - 1, y))
    } else if !grid.contains(&(x + 1, y)) {
        Some((x + 1, y))
    } else {
        None
    }
}

fn parse(line: &str) -> IResult<&str, Vec<(i32, i32)>> {
    let (rest, list) = separated_list1(tag(" -> "), tuple((digit1, char(','), digit1)))(line)?;
    Ok((
        rest,
        list.into_iter()
            .map(|(x, _, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .collect(),
    ))
}

fn bounds(grid: &Grid) -> [(i32, i32); 2] {
    grid.iter().fold(
        [(i32::MAX, i32::MIN); 2],
        |[(xmin, xmax), (ymin, ymax)], &(x, y)| {
            [(xmin.min(x), xmax.max(x)), (ymin.min(y), ymax.max(y))]
        },
    )
}
