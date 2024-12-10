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
        .filter_map(|(expected, operands)| solve1(*expected, operands).then_some(expected))
        .sum()
}

fn solve1(expected: i64, operands: &[i64]) -> bool {
    assert!(operands.len() != 0);
    if operands.len() == 1 {
        return operands[0] == expected;
    }
    let last = operands.last().unwrap();
    let rest = &operands[..operands.len() - 1];
    let mut result = false;
    if expected % last == 0 {
        // we can't exclude mul for this operand
        result = solve1(expected / last, rest)
    }
    result || solve1(expected - last, rest)
}

fn part2(equations: &[(i64, Vec<i64>)]) -> i64 {
    equations
        .iter()
        .filter_map(|(expected, operands)| solve2(*expected, operands).then_some(expected))
        .sum()
}

fn solve2(expected: i64, operands: &[i64]) -> bool {
    assert!(operands.len() != 0);
    let last = *operands.last().unwrap();
    let rest = &operands[..operands.len() - 1];
    if rest.is_empty() {
        return expected == last;
    }

    let mut solvable = false;
    if let Some(new) = remove_suffix(expected, last) {
        // we can't exclude concat
        solvable = solvable || solve2(new, rest);
    }
    if expected % last == 0 {
        // we can't exclude mul
        solvable = solvable || solve2(expected / last, rest);
    }
    solvable = solvable || solve2(expected - last, rest);
    solvable
}

fn remove_suffix(num: i64, suffix: i64) -> Option<i64> {
    if suffix == 0 {
        return (num.rem_euclid(10) == 0).then_some(num.div_euclid(10));
    }
    let digits = suffix.ilog10() + 1;
    let divisor = 10i64.pow(digits);
    let dividend = num - suffix;
    (dividend.rem_euclid(divisor) == 0).then_some(dividend.div_euclid(divisor))
}

#[cfg(test)]
mod tests {
    use crate::remove_suffix;

    #[test]
    fn test_remove_suffix() {
        let num = 12345;
        assert_eq!(remove_suffix(num, 5), Some(1234));
        assert_eq!(remove_suffix(num, 45), Some(123));
        assert_eq!(remove_suffix(num, 345), Some(12));
        assert_eq!(remove_suffix(num, 2345), Some(1));
        assert_eq!(remove_suffix(num, 12345), Some(0));

        assert_eq!(remove_suffix(num, 22345), None);

        assert_eq!(remove_suffix(num, 0), None);
        assert_eq!(remove_suffix(num, 1), None);
        assert_eq!(remove_suffix(num, 2), None);
        assert_eq!(remove_suffix(num, 3), None);
        assert_eq!(remove_suffix(num, 4), None);

        assert_eq!(remove_suffix(num, 6), None);
        assert_eq!(remove_suffix(num, 7), None);
        assert_eq!(remove_suffix(num, 8), None);
        assert_eq!(remove_suffix(num, 9), None);
    }
}
