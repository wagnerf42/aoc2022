use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
    path::Path,
};

fn load_map<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<Vec<u8>>> {
    Ok(BufReader::new(File::open(path)?)
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|c| match c {
                    b'a'..=b'z' => *c,
                    b'E' => b'z' + 1,
                    b'S' => b'a' - 1,
                    _ => panic!("invalid char in input"),
                })
                .collect()
        })
        .collect())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn neighbours(&self, xmax: usize, ymax: usize) -> impl Iterator<Item = Pos> {
        self.x
            .checked_sub(1)
            .map(|x| Pos { x, y: self.y })
            .into_iter()
            .chain(self.y.checked_sub(1).map(|y| Pos { x: self.x, y }))
            .chain((self.x + 1 < xmax).then_some(Pos {
                x: self.x + 1,
                y: self.y,
            }))
            .chain((self.y + 1 < ymax).then_some(Pos {
                x: self.x,
                y: self.y + 1,
            }))
    }
}

fn height_diff(grid: &[Vec<u8>], pos1: &Pos, pos2: &Pos) -> i8 {
    let h1 = grid[pos1.y][pos1.x];
    let h2 = grid[pos2.y][pos2.x];
    h2 as i8 - h1 as i8
}

fn find_pos(map: &[Vec<u8>], target: u8) -> Option<Pos> {
    map.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, code)| (Pos { x, y }, code))
        })
        .find_map(|(pos, code)| if *code == target { Some(pos) } else { None })
}

fn main() -> std::io::Result<()> {
    let map = load_map("input")?;
    let start = find_pos(&map, b'a' - 1).unwrap();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let mut seen = HashSet::new();
    let distance_to_end = loop {
        if let Some((pos, d)) = queue.pop_front() {
            if seen.contains(&pos) {
                continue;
            }
            seen.insert(pos);
            if map[pos.y][pos.x] == b'z' + 1 {
                break Some(d);
            } else {
                queue.extend(pos.neighbours(map[0].len(), map.len()).filter_map(|n| {
                    if !seen.contains(&n) && height_diff(&map, &pos, &n) <= 1 {
                        Some((n, d + 1))
                    } else {
                        None
                    }
                }))
            }
        } else {
            break None;
        }
    };
    println!("end is at {distance_to_end:?}");

    let start = find_pos(&map, b'z' + 1).unwrap();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let mut seen = HashSet::new();
    let distance_to_end = loop {
        if let Some((pos, d)) = queue.pop_front() {
            if seen.contains(&pos) {
                continue;
            }
            seen.insert(pos);
            if map[pos.y][pos.x] <= b'a' {
                break Some(d);
            } else {
                queue.extend(pos.neighbours(map[0].len(), map.len()).filter_map(|n| {
                    if !seen.contains(&n) && height_diff(&map, &pos, &n) >= -1 {
                        Some((n, d + 1))
                    } else {
                        None
                    }
                }))
            }
        } else {
            break None;
        }
    };
    println!("end is at {distance_to_end:?}");

    Ok(())
}
