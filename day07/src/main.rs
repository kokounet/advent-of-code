use anyhow::Error;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("day07/input.txt")?;
    let lines = content
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>());

    let dirs = parse(lines);

    println!("{}", part1(&dirs));
    println!("{}", part2(&dirs));
    Ok(())
}

fn parse<'a, T: Iterator<Item = Vec<&'a str>>>(lines: T) -> HashMap<String, u32> {
    lines
        .fold(
            (HashMap::new(), vec!["root"]),
            |(mut dirs, mut cwd), line| {
                match line.as_slice() {
                    ["$", "cd", "/"] => cwd = vec!["root"],
                    ["$", "cd", ".."] => cwd.pop().map_or((), |_| ()),
                    ["$", "cd", dir] => cwd.push(dir),
                    ["$", _] => {}
                    ["dir", _] => {}
                    [size, _] => {
                        let size: u32 = size.parse().unwrap();
                        for i in 1..=cwd.len() {
                            dirs.entry(cwd[..i].join("/"))
                                .and_modify(|s| *s += size)
                                .or_insert(size);
                        }
                    }
                    [..] => {}
                };
                (dirs, cwd)
            },
        )
        .0
}

fn part1(dirs: &HashMap<String, u32>) -> u32 {
    dirs.values().filter(|&&size| size < 100_000u32).sum()
}

fn part2(dirs: &HashMap<String, u32>) -> u32 {
    let available = 70_000_000 - dirs["root"];
    let update = 30_000_000u32;

    dirs.values()
        .filter(|&&size| (size + available) > update)
        .min()
        .unwrap()
        .to_owned()
}
