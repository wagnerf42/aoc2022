use len_trait::len::*;
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufReader, Read},
    ops::{Index, IndexMut},
};

fn main() -> std::io::Result<()> {
    let mut moves = Vec::new();
    BufReader::new(File::open("input")?).read_to_end(&mut moves)?;
    moves.retain(|&c| c == b'<' || c == b'>');

    let rocks = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];

    let mut map = CMap::new(); // let's always keep 7 lines completely free

    let mut movements = moves
        .iter()
        .map(|c| match c {
            b'<' => -1,
            b'>' => 1,
            _ => unreachable!(),
        })
        .cycle();

    for rock in rocks.iter().cycle().take(2021) {
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

    let mut movements = moves
        .iter()
        .map(|c| match c {
            b'<' => -1,
            b'>' => 1,
            _ => unreachable!(),
        })
        .cycle()
        .enumerate();

    let mut map = CMap::new();
    // let's remember all places where a horizontal rock (index 0) fills a line
    let mut seen_pos: HashMap<(VecDeque<[u8; 7]>, usize), usize> = HashMap::new();
    let mut remembered_positions: Vec<(VecDeque<[u8; 7]>, usize, usize, usize)> = Vec::new();
    for (rock_index, rock) in rocks.iter().cycle().take(10_000_000_000).enumerate() {
        let mut pos = (2, map.len() - 4);
        for (movement_index, x_offset) in &mut movements {
            pos = try_side_move(rock, pos, &map, x_offset);
            if let Some(down_pos) = try_moving_down(rock, pos, &map) {
                pos = down_pos
            } else {
                add_to_map(&mut map, rock, pos);
                if map.line_full(pos.1) {
                    map.compress(pos.1);
                    if rock_index % rocks.len() == 0 {
                        let new_index = remembered_positions.len();
                        remembered_positions.push((
                            map.lines.clone(),
                            movement_index,
                            pos.1,
                            rock_index,
                        ));
                        let move_code = movement_index % moves.len();
                        if let Some(seen_index) = seen_pos.get(&(map.lines.clone(), move_code)) {
                            eprintln!("at pos {new_index} we go back to pos {seen_index}");
                            complete_computations(
                                &moves,
                                &rocks,
                                &remembered_positions[*seen_index..],
                                movement_index,
                                pos.1,
                                rock_index,
                            );
                            return Ok(());
                        }
                        seen_pos.insert((map.lines.clone(), move_code), new_index);
                    }
                }
                break;
            }
        }
    }
    println!("tower height: {}", map.len() - 7);
    Ok(())
}

fn complete_computations(
    moves: &[u8],
    rocks: &[Vec<(usize, usize)>],
    seen_index: &[(VecDeque<[u8; 7]>, usize, usize, usize)],
    mut movement_index: usize, // we just did this guy
    mut y: usize,              // we just completed the line at this y
    rock_index: usize,         // we just stored this rock
) {
    eprintln!("cycle is of length {}", seen_index.len());
    let mut needed_rocks = 1_000_000_000_000 - (rock_index + 1); // this is a shitty +1
    let (first_map, first_move_index, first_y, first_rock_index) = seen_index.first().unwrap();
    let (_, last_move_index, last_y, last_rock_index) = seen_index.last().unwrap();
    let rocks_per_full_cycle = last_rock_index - first_rock_index;
    let y_per_full_cycle = last_y - first_y;
    let movements_per_full_cycle = last_move_index - first_move_index;
    eprintln!("each full cycle we do {rocks_per_full_cycle} and we need {needed_rocks}");
    let full_cycles_needed = needed_rocks / rocks_per_full_cycle;
    eprintln!("we need {full_cycles_needed} full cycles");
    needed_rocks %= rocks_per_full_cycle;
    movement_index += movements_per_full_cycle * full_cycles_needed;
    y += y_per_full_cycle * full_cycles_needed;
    eprintln!("we are now at {movement_index} {y} {needed_rocks}");
    let mut map = CMap {
        real_height: y + first_map.len(),
        lines: first_map.clone(),
    };

    let mut movements = moves
        .iter()
        .map(|c| match c {
            b'<' => -1,
            b'>' => 1,
            _ => unreachable!(),
        })
        .cycle()
        .skip((movement_index + 1) % moves.len());

    for rock in rocks.iter().cycle().skip(1).take(needed_rocks) {
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
    fn display(&self) {
        self.lines
            .iter()
            .rev()
            .map(|l| std::str::from_utf8(l).unwrap())
            .for_each(|l| println!("{l}"))
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
