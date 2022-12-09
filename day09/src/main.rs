use anyhow::Error;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(pub i32, pub i32);

#[derive(Debug)]
struct Rope<const SIZE: usize> {
    pub knots: [Pos; SIZE],
}

fn main() -> Result<(), Error> {
    let content = fs::read_to_string("day09/input.txt")?;
    let commands: Vec<_> = content
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace();
            let dir: Pos = match it.next().unwrap() {
                "U" => Pos(0, 1),
                "D" => Pos(0, -1),
                "L" => Pos(-1, 0),
                "R" => Pos(1, 0),
                _ => unreachable!(),
            };
            let amount: i32 = it.next().unwrap().parse().unwrap();
            (dir, amount)
        })
        .collect();
    println!("{}", simulate::<2>(&commands));
    println!("{}", simulate::<10>(&commands));
    Ok(())
}

fn simulate<const SIZE: usize>(commands: &[(Pos, i32)]) -> usize {
    let (visited, _) = commands.iter().fold(
        (HashSet::new(), Rope::<SIZE>::new()),
        |(mut visited, mut rope), &(dir, amount)| {
            for _ in 0..amount {
                rope.slide(dir);
                visited.insert(rope.tail());
            }
            (visited, rope)
        },
    );
    visited.len()
}

impl<const SIZE: usize> Rope<SIZE> {
    pub fn new() -> Self {
        Self {
            knots: std::array::from_fn(|_| Pos::default()),
        }
    }

    pub fn slide(&mut self, dir: Pos) {
        self.knots[0] = Pos(self.knots[0].0 + dir.0, self.knots[0].1 + dir.1);
        for i in 1..SIZE {
            self.knots[i].follow(self.knots[i - 1]);
        }
    }

    pub fn tail(&self) -> Pos {
        self.knots[SIZE - 1]
    }
}

impl Pos {
    fn follow(&mut self, other: Pos) {
        let Pos(x, y) = Pos(other.0 - self.0, other.1 - self.1);
        if x.abs().max(y.abs()) != 2 {
            return;
        }
        *self = Pos(self.0 + x.signum(), self.1 + y.signum());
    }
}
