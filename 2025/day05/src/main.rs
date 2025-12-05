use anyhow::{anyhow, Result};
use common::time;

fn main() -> Result<()> {
    let content = std::fs::read_to_string("day05/input.txt")?;
    let mut it = content.lines();
    let ranges = it
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (start, end) = line
                .split_once("-")
                .ok_or(anyhow!("Unexpected range: {line}"))?;
            let start = start.parse::<i64>()?;
            let end = end.parse::<i64>()?;
            Ok((start, end))
        })
        .collect::<Result<Vec<_>>>()?;
    let ids = it
        .map(|line| line.parse::<i64>().map_err(|e| anyhow!(e)))
        .collect::<Result<Vec<_>>>()?;

    println!("{}", time!(part1(&ranges, &ids)));
    println!("{}", time!(part2(ranges)));
    Ok(())
}

fn part1(ranges: &[(i64, i64)], ids: &[i64]) -> i64 {
    ids.iter()
        .filter(|id| ranges.iter().any(|&(a, b)| (a..=b).contains(id)))
        .count() as i64
}

fn part2(mut ranges: Vec<(i64, i64)>) -> i64 {
    assert!(!ranges.is_empty());
    // sort by lower bound
    ranges.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
    let mut it = ranges.into_iter();
    let mut merged = Vec::new();
    let mut prev = it.next().expect("ranges can't be empty");
    for curr in it {
        if curr.0 <= prev.1 {
            // grow current interval
            prev.1 = prev.1.max(curr.1);
        } else {
            // create a new interval
            merged.push(prev);
            prev = curr;
        }
    }
    // push the last merged interval
    merged.push(prev);
    merged.into_iter().map(|(a, b)| b - a + 1).sum()
}
