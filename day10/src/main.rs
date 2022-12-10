use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let content = fs::read_to_string("day10/input.txt")?;
    let positions = content
        .lines()
        .map(|l| l.split_whitespace().collect())
        .fold(vec![1], |mut acc, inst: Vec<_>| {
            let &value = acc.last().unwrap();
            acc.push(value);
            match inst.as_slice() {
                ["addx", val] => acc.push(value + val.parse::<i32>().unwrap()),
                [..] => {}
            };
            acc
        });

    println!("{}", part1(&positions));
    println!("{}", part2(&positions));
    Ok(())
}

fn part1(positions: &[i32]) -> i32 {
    (20..)
        .step_by(40)
        .take_while(|&i| i < positions.len())
        .map(|i| i as i32 * positions[i - 1])
        .sum()
}

fn part2(positions: &[i32]) -> String {
    let mut screen = String::with_capacity(6 * 40);
    for (cycle, &sprite) in positions.iter().enumerate().take(6 * 40) {
        let cursor = cycle % 40;
        if cursor == 0 && cycle > 0 {
            screen.push('\n');
        }
        if (cursor as i32 - sprite).abs() <= 1 {
            screen.push('█');
        } else {
            screen.push(' ');
        }
    }
    screen
}
