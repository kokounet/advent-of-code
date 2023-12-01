mod pos;
mod sphere;

use anyhow::Result;
use regex::Regex;
use std::ops::RangeInclusive;
use std::time::Instant;
use std::{collections::HashSet, fs};

use crate::pos::Pos;
use crate::sphere::Sphere;

fn main() -> Result<()> {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")?;

    let content = fs::read_to_string("day15/input.txt")?;
    let (spheres, beacons): (HashSet<_>, HashSet<_>) = content
        .lines()
        .map(|l| re.captures(l).unwrap())
        .map(|cap| {
            cap.iter()
                .skip(1)
                .filter_map(|m| m.map(|num| num.as_str().parse::<i64>().unwrap()))
                .collect::<Vec<_>>()
        })
        .map(|rec| [(rec[0], rec[1]).into(), (rec[2], rec[3]).into()])
        .map(|[sensor, beacon]: [Pos; 2]| {
            (
                Sphere {
                    center: sensor,
                    radius: sensor.distance(&beacon),
                },
                beacon,
            )
        })
        .unzip();
    let now = Instant::now();
    println!("{}", part1(&spheres, &beacons));
    println!("{}", part2(&spheres));
    println!("{:?}", now.elapsed());
    Ok(())
}

fn part1(spheres: &HashSet<Sphere>, beacons: &HashSet<Pos>) -> i64 {
    let y = 10;
    let beacons = beacons.iter().filter(|pos| pos.y == y).count() as i64;
    scan_y(spheres, &y, i64::MIN..=i64::MAX)
        .iter()
        .map(len)
        .sum::<i64>()
        - beacons
}

fn part2(spheres: &HashSet<Sphere>) -> i64 {
    let bounds = 0..=4_000_000;
    let limit = len(&bounds);
    let (y, scan) = bounds
        .clone()
        .map(|y| (y, scan_y(spheres, &y, bounds.clone())))
        .find(|(_, scan)| scan.iter().map(len).sum::<i64>() < limit)
        .unwrap();
    // Copium it won't be on the border
    let x = scan[0].end() + 1;
    x * 4_000_000 + y
}

fn scan_y(
    spheres: &HashSet<Sphere>,
    y: &i64,
    bounds: RangeInclusive<i64>,
) -> Vec<RangeInclusive<i64>> {
    let mut intervals: Vec<_> = spheres
        .iter()
        .filter_map(|sphere| {
            sphere
                .raycast_y(y)
                .and_then(|iv| intersection(&iv, &bounds))
        })
        .collect();
    intervals.sort_by_key(|iv| *iv.start());
    collapse(intervals)
}

/// Collapse a series of sorted interval into their minimal set in O(n)
fn collapse(intervals: Vec<RangeInclusive<i64>>) -> Vec<RangeInclusive<i64>> {
    if intervals.is_empty() {
        return vec![];
    }
    let mut collapsed = Vec::with_capacity(intervals.len());
    let mut it = intervals.into_iter();
    collapsed.push(it.next().unwrap());
    for (mut start, mut stop) in it.map(|iv| iv.into_inner()) {
        let last = collapsed.last().unwrap().clone();
        if last.contains(&start) {
            collapsed.pop();
            start = *last.start();
            stop = stop.max(*last.end());
        }
        collapsed.push(start..=stop);
    }
    collapsed
}

fn intersection(
    lhs: &RangeInclusive<i64>,
    rhs: &RangeInclusive<i64>,
) -> Option<RangeInclusive<i64>> {
    let &start = lhs.start().max(rhs.start());
    let &end = lhs.end().min(rhs.end());
    if start > end {
        return None;
    }
    Some(start..=end)
}

fn len(interval: &RangeInclusive<i64>) -> i64 {
    interval.end() - interval.start() + 1
}
