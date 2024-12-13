use std::{cmp::Ordering, collections::BTreeSet, fmt};

fn main() -> anyhow::Result<()> {
    let content = std::fs::read_to_string("day09/input.txt")?;
    let (_, disk) =
        content
            .chars()
            .enumerate()
            .fold((0i32, Disk::default()), |(cur, mut disk), (i, c)| {
                let is_file = i & 1 == 0; // even indices are free space
                let len = c.to_digit(10).unwrap() as i32;
                if len == 0 {
                    return (cur, disk);
                }
                if is_file {
                    disk.full.insert((cur, len, i as i32 / 2));
                } else {
                    disk.free.insert((cur, len));
                }
                (cur + len, disk)
            });
    println!("{}", part1(disk.clone()));
    println!("{}", part2(disk));

    Ok(())
}

#[derive(Debug, Default, Clone)]
struct Disk {
    pub full: BTreeSet<(i32, i32, i32)>,
    pub free: BTreeSet<(i32, i32)>,
}

impl Disk {
    fn checksum(&self) -> u64 {
        self.full
            .iter()
            .map(|&(start, len, file_id)| {
                // start*file_id + ... + (start+ len - 1)*file_id
                // = file_id*(start + (start+1) + ... + (start + len - 1))
                // = file_id*(len*start + (1+2+...+len-1))
                // = file_id*(len*start + (len*(len-1)/2))
                let start = start as u64;
                let len = len as u64;
                let file_id = file_id as u64;
                let score = file_id * (len * start + len * (len - 1) / 2);
                score
            })
            .sum()
    }
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let flat = self
            .full
            .iter()
            .map(|&(start, len, id)| (start, len, char::from_digit(id as u32, 36).unwrap()))
            .chain(self.free.iter().map_while(|&(start, len)| {
                // ignore remaining space at the end
                let last = self.full.last()?.0;
                (start < last).then_some((start, len, '.'))
            }))
            .collect::<BTreeSet<_>>();
        let mut cur = 0;
        for (start, len, c) in flat {
            assert_eq!(cur, start);
            for _ in 0..len {
                write!(f, "{c}")?;
            }
            cur += len;
        }
        Ok(())
    }
}

fn part1(mut disk: Disk) -> u64 {
    while let Some((start, free_size)) = disk.free.pop_first() {
        let Some((pos, file_size, file_id)) = disk.full.pop_last() else {
            break;
        };
        if pos < start {
            disk.full.insert((pos, file_size, file_id));
            break;
        }
        disk.full.insert((start, free_size.min(file_size), file_id));
        match free_size.cmp(&file_size) {
            Ordering::Less => {
                let new_size = file_size - free_size;
                disk.full.insert((pos, new_size, file_id));
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                disk.free.insert((start + file_size, free_size - file_size));
            }
        }
    }
    disk.checksum()
}

fn part2(mut disk: Disk) -> u64 {
    let full = disk.full.clone();
    for (file_pos, file_size, file_id) in full.into_iter().rev() {
        let Some(&(free_pos, free_size)) = disk
            .free
            .iter()
            .find(|&&(pos, size)| size >= file_size && pos < file_pos)
        else {
            continue;
        };
        disk.free.remove(&(free_pos, free_size));

        disk.full.remove(&(file_pos, file_size, file_id));
        disk.full.insert((free_pos, file_size, file_id));

        disk.free.insert((file_pos, file_size));
        if free_size > file_size {
            disk.free
                .insert((free_pos + file_size, free_size - file_size));
        }
    }
    disk.checksum()
}
