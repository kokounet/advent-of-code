use anyhow::{bail, Context, Result};

fn main() -> Result<()> {
    let content = std::fs::read_to_string("day07/input.txt")?;
    let equations = content
        .lines()
        .map(|line| {
            let Some((res, rest)) = line.split_once(":") else {
                bail!("no `:` in line");
            };
            let res = res.trim().parse::<i64>().context("not an int")?;
            let rest = rest
                .trim()
                .split_whitespace()
                .map(|e| e.parse::<i64>().context("not an int"))
                .collect::<Result<Vec<_>>>()?;
            Ok((res, rest))
        })
        .collect::<Result<Vec<_>>>()?;
    println!("{}", part1(&equations));
    println!("{}", part2(&equations));
    Ok(())
}

fn part1(equations: &[(i64, Vec<i64>)]) -> i64 {
    equations
        .iter()
        .filter_map(|(expected, operands)| {
            let num_ops = operands.len() - 1;
            (0..(1 << num_ops))
                .map(|bits| {
                    let mut it = operands.iter().cloned();
                    let mut res = it.next().unwrap();
                    for (i, e) in it.enumerate() {
                        if (bits >> i) & 1 == 1 {
                            res *= e;
                        } else {
                            res += e;
                        }
                    }
                    res
                })
                .find(|res| res == expected)
        })
        .sum()
}

fn part2(equations: &[(i64, Vec<i64>)]) -> i64 {
    equations
        .iter()
        .filter_map(|(expected, operands)| {
            let num_ops = operands.len() - 1;
            (0..(3i64.pow(num_ops as u32)))
                .map(|mask| {
                    let mut it = operands.iter().cloned();
                    let mut res = it.next().unwrap();
                    for (i, e) in it.enumerate() {
                        let flag = (mask / 3i64.pow(i as u32)) % 3;
                        if flag == 0 {
                            res += e;
                        } else if flag == 1 {
                            res *= e;
                        } else if flag == 2 {
                            res = format!("{res}{e}").parse().unwrap(); // yummy
                        }
                    }
                    res
                })
                .find(|res| res == expected)
        })
        .sum()
}
