use std::collections::HashSet;

use glam::IVec2;

fn main() -> anyhow::Result<()> {
    let content = std::fs::read_to_string("day06/input.txt")?;
    let lines = content.lines().collect::<Vec<_>>();
    let (w, h) = (lines[0].len() as i32, lines.len() as i32);
    let (start, grid) = lines
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (x as i32, y as i32, c))
        })
        .fold(
            (IVec2::ZERO, Grid::new(w, h)),
            |(mut start, mut grid), (x, y, c)| {
                match c {
                    '#' => {
                        grid.obstacles.insert(IVec2::new(x, y));
                    }
                    '^' => {
                        start = IVec2::new(x, y);
                    }
                    _ => {}
                }
                (start, grid)
            },
        );
    println!("{}", part1(start, &grid));
    println!("{}", part2(start, grid));
    Ok(())
}

#[derive(Debug, Clone)]
struct Grid {
    pub dims: IVec2,
    pub obstacles: HashSet<IVec2>,
}

impl Grid {
    pub fn new(w: i32, h: i32) -> Self {
        Grid {
            dims: IVec2::new(w, h),
            obstacles: Default::default(),
        }
    }

    pub fn contains(&self, point: IVec2) -> bool {
        point.cmpge(IVec2::ZERO).all() && point.cmplt(self.dims).all()
    }

    pub fn walk(&self, start: IVec2, dir: IVec2) -> Result<IVec2, Hit> {
        let new = start + dir;
        // bound check
        if !self.contains(new) {
            return Err(Hit::OutOfBound);
        }
        // obstable check
        if self.obstacles.contains(&new) {
            return Err(Hit::Obstable);
        }
        Ok(new)
    }

    pub fn visit(&self, start: IVec2) -> Result<HashSet<(IVec2, IVec2)>, HashSet<(IVec2, IVec2)>> {
        let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)].map(IVec2::from);
        let mut visited = HashSet::new();
        let mut curr = start;
        for dir in dirs.into_iter().cycle() {
            loop {
                if !visited.insert((curr, dir)) {
                    // hit a cycle if we're at the same position as previously
                    // with the same direction
                    return Err(visited);
                }
                match self.walk(curr, dir) {
                    Ok(pos) => {
                        curr = pos;
                    }
                    Err(Hit::Obstable) => {
                        break; // change direction
                    }
                    Err(Hit::OutOfBound) => return Ok(visited),
                }
            }
        }
        unreachable!("somehow broke out of an infinite loop")
    }
}

enum Hit {
    Obstable,
    OutOfBound,
}

fn part1(start: IVec2, grid: &Grid) -> i32 {
    grid.visit(start)
        .unwrap_or_else(|e| e)
        .into_iter()
        .map(|(pos, _)| pos)
        .collect::<HashSet<_>>() // removes duplicates pos
        .len() as i32
}

fn part2(start: IVec2, mut grid: Grid) -> i32 {
    let mut points: HashSet<_> = grid
        .visit(start)
        .unwrap()
        .into_iter()
        .map(|(pos, _)| pos)
        .collect();
    points.remove(&start); // cant place an obstacle at the start
    points
        .into_iter()
        .filter(|&point| {
            grid.obstacles.insert(point);
            let visited = grid.visit(start);
            grid.obstacles.remove(&point);
            visited.is_err() // detected a cycle
        })
        .count() as i32
}
