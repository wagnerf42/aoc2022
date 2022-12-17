use len_trait::len::*;
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufReader, Read},
    ops::{Index, IndexMut},
};

fn main() -> std::io::Result<()> {
    let mut moves = Vec::new();
    BufReader::new(File::open("input")?).read_to_end(&mut moves)?;

    let rocks = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];

    let mut map = vec![[b'.'; 7]; 7]; // let's always keep 7 lines completely free

    let mut movements = moves
        .iter()
        .filter_map(|c| match c {
            b'<' => Some(-1),
            b'>' => Some(1),
            _ => None,
        })
        .cycle();

    for rock in rocks.iter().cycle().take(2022) {
        let mut pos = (2, map.len() - 4);
        for x_offset in &mut movements {
            pos = try_side_move(rock, pos, &map, x_offset);
            if let Some(down_pos) = try_moving_down(rock, pos, &map) {
                pos = down_pos
            } else {
                add_to_map(&mut map, rock, pos);
                break;
            }
        }
    }
    println!("tower height: {}", map.len() - 7);

    let mut map = CMap::new();
    let mut movements = moves
        .iter()
        .filter_map(|c| match c {
            b'<' => Some(-1),
            b'>' => Some(1),
            _ => None,
        })
        .cycle();

    for (i, rock) in rocks.iter().cycle().take(1_000_000_000_000).enumerate() {
        if i % 1_000_000_000 == 0 {
            println!(
                "doing {}, size is now {}",
                i / 1_000_000_000,
                map.lines.len()
            );
        }
        let mut pos = (2, map.len() - 4);
        for x_offset in &mut movements {
            pos = try_side_move(rock, pos, &map, x_offset);
            if let Some(down_pos) = try_moving_down(rock, pos, &map) {
                pos = down_pos
            } else {
                add_to_map(&mut map, rock, pos);
                map.compress(pos.1);
                break;
            }
        }
    }
    println!("tower height: {}", map.len() - 7);

    Ok(())
}

fn stone_positions(
    rock: &[(usize, usize)],
    pos: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> + '_ {
    rock.iter().map(move |(x, y)| (x + pos.0, y + pos.1))
}

fn try_side_move<V: Index<usize, Output = [u8; 7]>>(
    rock: &[(usize, usize)],
    pos: (usize, usize),
    map: &V,
    x_offset: isize,
) -> (usize, usize) {
    if let Some(new_x) = pos.0.checked_add_signed(x_offset) {
        if stone_positions(rock, (new_x, pos.1)).all(|(x, y)| x < 7 && map[y][x] == b'.') {
            (new_x, pos.1)
        } else {
            pos
        }
    } else {
        pos
    }
}

fn try_moving_down<V: Index<usize, Output = [u8; 7]>>(
    rock: &[(usize, usize)],
    pos: (usize, usize),
    map: &V,
) -> Option<(usize, usize)> {
    if let Some(new_y) = pos.1.checked_add_signed(-1) {
        if stone_positions(rock, (pos.0, new_y)).all(|(x, y)| map[y][x] == b'.') {
            Some((pos.0, new_y))
        } else {
            None
        }
    } else {
        None
    }
}

fn add_to_map<V: Extend<[u8; 7]> + Index<usize, Output = [u8; 7]> + IndexMut<usize> + Len>(
    map: &mut V,
    rock: &[(usize, usize)],
    pos: (usize, usize),
) {
    let y_max = stone_positions(rock, pos)
        .map(|(x, y)| {
            map[y][x] = b'#';
            y
        })
        .max()
        .unwrap();
    while map.len() - y_max < 8 {
        map.extend(std::iter::once([b'.'; 7]));
    }
}

struct CMap {
    real_height: usize,
    lines: VecDeque<[u8; 7]>,
}

impl CMap {
    fn new() -> Self {
        CMap {
            real_height: 7,
            lines: std::iter::repeat_with(|| [b'.'; 7]).take(7).collect(),
        }
    }
    fn deque_y(&self, real_y: usize) -> usize {
        real_y - (self.real_height - self.lines.len())
    }
    fn compress(&mut self, y: usize) {
        if let Some(full_line_y) = (y..(y + 4)).filter(|&y| self.line_full(y)).last() {
            // remove all lines until here
            let y = self.deque_y(full_line_y);
            self.lines.drain(..y);
        }
    }
    fn line_full(&self, y: usize) -> bool {
        self.lines[self.deque_y(y)].iter().all(|&c| c == b'#')
    }
}

impl Extend<[u8; 7]> for CMap {
    fn extend<T: IntoIterator<Item = [u8; 7]>>(&mut self, iter: T) {
        let old_lines_num = self.lines.len();
        self.lines.extend(iter);
        self.real_height += self.lines.len() - old_lines_num;
    }
}

impl Index<usize> for CMap {
    type Output = [u8; 7];

    fn index(&self, index: usize) -> &Self::Output {
        &self.lines[self.deque_y(index)]
    }
}

impl IndexMut<usize> for CMap {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let y = self.deque_y(index);
        &mut self.lines[y]
    }
}

impl Empty for CMap {
    fn is_empty(&self) -> bool {
        self.real_height == 0
    }
}

impl Len for CMap {
    fn len(&self) -> usize {
        self.real_height
    }
}
