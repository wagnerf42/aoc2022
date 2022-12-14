use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Map {
    pixels: Vec<Vec<char>>,
}

impl Map {
    fn new(xmax: usize, ymax: usize) -> Self {
        Map {
            pixels: std::iter::repeat_with(|| vec!['.'; xmax + 1])
                .take(ymax + 1)
                .collect(),
        }
    }
    fn add(&mut self, content: char, position: (usize, usize)) {
        self.pixels[position.1][position.0] = content
    }
    fn valid_pos(&self, x: isize, y: isize) -> bool {
        0 <= x && x < self.pixels[0].len() as isize && 0 <= y && y < self.pixels.len() as isize
    }
    fn free_space(&self, x: isize, y: isize) -> bool {
        self.pixels[y as usize][x as usize] == '.'
    }
    fn add_sand(&mut self, mut x: isize, mut y: isize) -> Option<(isize, isize)> {
        'outer: loop {
            let possible_successors = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)];
            for (new_x, new_y) in possible_successors {
                if !self.valid_pos(new_x, new_y) {
                    return None;
                }
                if self.free_space(new_x, new_y) {
                    x = new_x;
                    y = new_y;
                    continue 'outer;
                }
            }
            self.add('o', (x as usize, y as usize));
            return Some((x, y));
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.pixels {
            f.write_str(&line.iter().collect::<String>())?;
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn rocks() -> impl Iterator<Item = (usize, usize)> {
    BufReader::new(File::open("input").unwrap())
        .lines()
        .filter_map(|line| line.ok())
        .flat_map(|line| {
            line.split(" -> ")
                .map(|point| {
                    point
                        .split(',')
                        .map(|coordinate| coordinate.parse::<usize>().unwrap())
                        .tuples()
                        .next()
                        .unwrap()
                })
                .tuple_windows()
                .flat_map(|((x1, y1), (x2, y2))| {
                    (x1.min(x2)..=x1.max(x2)).cartesian_product(y1.min(y2)..=y1.max(y2))
                })
                .collect::<Vec<_>>() // that's really a recurring pb
        })
}

fn main() -> std::io::Result<()> {
    let (xmin, xmax) = rocks().map(|(x, _)| x).minmax().into_option().unwrap();
    let ymax = rocks().map(|(_, y)| y).max().unwrap();
    let mut map = Map::new(xmax - xmin, ymax);
    for (x, y) in rocks() {
        map.add('#', (x - xmin, y));
    }
    // println!("{map}");
    let mut count = 0;
    while map.add_sand(500 - xmin as isize, 0).is_some() {
        // println!("{map}");
        count += 1;
    }
    println!("we added {count}");

    let xmin = xmin.min(500 - (ymax + 2));
    let xmax = xmax.max(500 + ymax + 2);
    let mut map = Map::new(xmax - xmin, ymax + 2);
    for (x, y) in rocks().chain(((500 - (ymax + 2))..=(500 + (ymax + 2))).map(|x| (x, ymax + 2))) {
        map.add('#', (x - xmin, y));
    }

    // println!("{map}");
    let mut count = 0;
    while let Some((x, y)) = map.add_sand(500 - xmin as isize, 0) {
        // println!("{map}");
        count += 1;
        if x + xmin as isize == 500 && y == 0 {
            break;
        }
    }
    println!("we added {count}");

    Ok(())
}
