use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Map {
    pixels: Vec<Vec<char>>,
}

impl Map {
    fn new() -> Self {
        Map { pixels: Vec::new() }
    }
    fn add(&mut self, content: char, position: (usize, usize)) {
        eprintln!("{position:?}");
    }
}

fn main() -> std::io::Result<()> {
    let mut map = Map::new();
    BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| {
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
                .for_each(|((x1, y1), (x2, y2))| {
                    if x1 == x2 {
                        (y1.min(y2)..=y1.max(y2))
                            .map(move |y| (x1, y))
                            .for_each(|p| map.add('x', p))
                    } else {
                        assert!(y1 == y2);
                        (x1.min(x2)..=x1.max(x2))
                            .map(move |x| (x, y1))
                            .for_each(|p| map.add('x', p))
                    }
                })
        });
    Ok(())
}
