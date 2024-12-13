use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let content = std::fs::read_to_string("day11/input.txt")?;
    let stones = content
        .split_whitespace()
        .map(|stone| stone.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;
    println!("{}", part1(&stones));
    println!("{}", part2(&stones));
    Ok(())
}

fn num_digits(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    (n.ilog10() + 1) as u64
}

fn split(n: u64) -> (u64, u64) {
    let k = num_digits(n);
    assert!(k & 1 == 0);
    let div = 10u64.pow(k as u32 / 2);
    (n.div_euclid(div), n.rem_euclid(div))
}

fn blink(stone: u64, blinks: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if let Some(&splits) = cache.get(&(stone, blinks)) {
        return splits;
    }
    if blinks == 0 {
        return 1;
    }
    let res = match stone {
        0 => blink(1, blinks - 1, cache),
        odd if num_digits(stone) & 1 == 1 => blink(2024 * odd, blinks - 1, cache),
        even => {
            let (left, right) = split(even);
            blink(left, blinks - 1, cache) + blink(right, blinks - 1, cache)
        }
    };
    cache.insert((stone, blinks), res);
    res
}

fn part1(stones: &[u64]) -> u64 {
    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|&stone| blink(stone, 25, &mut cache))
        .sum()
}

fn part2(stones: &[u64]) -> u64 {
    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|&stone| blink(stone, 75, &mut cache))
        .sum()
}
