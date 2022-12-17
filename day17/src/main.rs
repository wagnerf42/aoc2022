use std::{
    fs::File,
    io::{BufReader, Read},
};

fn display_map(map: &[[char; 7]]) {
    println!("*****************");
    for line in map.iter().rev() {
        println!("{}", line.iter().collect::<String>())
    }
}

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

    let mut map = vec![['.'; 7]; 7]; // let's always keep 7 lines completely free

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
                // display_map(&map);
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

fn try_side_move(
    rock: &[(usize, usize)],
    pos: (usize, usize),
    map: &[[char; 7]],
    x_offset: isize,
) -> (usize, usize) {
    if let Some(new_x) = pos.0.checked_add_signed(x_offset) {
        if stone_positions(rock, (new_x, pos.1)).all(|(x, y)| x < 7 && map[y][x] == '.') {
            (new_x, pos.1)
        } else {
            pos
        }
    } else {
        pos
    }
}

fn try_moving_down(
    rock: &[(usize, usize)],
    pos: (usize, usize),
    map: &[[char; 7]],
) -> Option<(usize, usize)> {
    if let Some(new_y) = pos.1.checked_add_signed(-1) {
        if stone_positions(rock, (pos.0, new_y)).all(|(x, y)| map[y][x] == '.') {
            Some((pos.0, new_y))
        } else {
            None
        }
    } else {
        None
    }
}

fn add_to_map(map: &mut Vec<[char; 7]>, rock: &[(usize, usize)], pos: (usize, usize)) {
    let y_max = stone_positions(rock, pos)
        .map(|(x, y)| {
            map[y][x] = '#';
            y
        })
        .max()
        .unwrap();
    while map.len() - y_max < 8 {
        map.push(['.'; 7]);
    }
}
