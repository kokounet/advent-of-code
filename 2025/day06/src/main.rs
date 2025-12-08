use anyhow::{anyhow, Error, Result};
use common::time;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
}

fn main() -> Result<()> {
    let content = std::fs::read_to_string("day06/input.txt")?;
    println!("{}", time!(solve(&parse1(&content)?)));
    println!("{}", time!(solve(&parse2(&content)?)));
    Ok(())
}

fn parse1(content: &str) -> Result<Vec<(Vec<i64>, Op)>> {
    let problems: Vec<Vec<_>> = content
        .lines()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();
    let lines = problems.len();
    let cols = problems[0].len();
    let mut res = Vec::with_capacity(cols);
    for c in 0..cols {
        let nums = (0..lines - 1)
            .map(|l| problems[l][c].parse::<i64>().map_err(Into::into))
            .collect::<Result<Vec<_>>>()?;
        let op = problems[lines - 1][c].try_into()?;
        res.push((nums, op))
    }
    Ok(res)
}

fn parse2(content: &str) -> Result<Vec<(Vec<i64>, Op)>> {
    let lines: Vec<Vec<_>> = content.lines().map(|l| l.chars().collect()).collect();
    assert!(!lines.is_empty());
    let cols = lines[0].len();
    let mut iters: Vec<_> = lines.into_iter().map(|line| line.into_iter()).collect();
    let mut transpose: Vec<Vec<_>> = (0..cols)
        .map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect())
        .collect();
    // read from right to left (technically not necessary as operators are commutative here)
    transpose.reverse();
    let mut res = vec![];
    let mut acc = vec![];
    let mut op = None;
    for e in transpose {
        let len = e.len();
        op = op.or_else(|| Op::try_from(&e[len - 1]).ok());
        match e[..len - 1]
            .iter()
            .collect::<String>()
            .trim()
            .parse::<i64>()
        {
            Ok(num) => acc.push(num),
            Err(_) => {
                assert!(op.is_some());
                res.push((acc.clone(), op.unwrap()));
                acc.clear();
                op = None;
            }
        }
    }
    res.push((acc, op.unwrap()));
    Ok(res)
}

fn solve(problems: &[(Vec<i64>, Op)]) -> i64 {
    problems
        .iter()
        .map(|(num, op)| match op {
            Op::Add => num.iter().sum::<i64>(),
            Op::Mul => num.iter().product(),
        })
        .sum()
}

impl TryFrom<&str> for Op {
    type Error = Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "*" => Ok(Self::Mul),
            "+" => Ok(Self::Add),
            _ => Err(anyhow!("Wrong operator: {value}")),
        }
    }
}

impl TryFrom<&char> for Op {
    type Error = Error;

    fn try_from(value: &char) -> std::result::Result<Self, Self::Error> {
        match value {
            '*' => Ok(Self::Mul),
            '+' => Ok(Self::Add),
            _ => Err(anyhow!("Wrong operator: {value}")),
        }
    }
}
