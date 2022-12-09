use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn after_move(&self, direction: (i32, i32), distance: i32) -> Pos {
        Pos {
            x: self.x + direction.0 * distance,
            y: self.y + direction.1 * distance,
        }
    }
    fn distance(&self, other: &Self) -> i32 {
        let xmin = self.x.min(other.x);
        let ymin = self.y.min(other.y);
        let xmax = self.x.max(other.x);
        let ymax = self.y.max(other.y);
        (xmax - xmin).max(ymax - ymin)
    }
    fn average(&self, other: &Self) -> Self {
        todo!()
    }
}

fn main() -> std::io::Result<()> {
    BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| {
            let mut tokens = l.split_whitespace();
            let direction = tokens.next().unwrap();
            let distance = tokens.next().unwrap().parse::<i32>().unwrap();
            (
                match direction {
                    "L" => (-1, 0),
                    "R" => (1, 0),
                    "U" => (0, 1),
                    "D" => (0, -1),
                    _ => panic!("unknown direction"),
                },
                distance,
            )
        })
        .scan(
            (Default::default(), Default::default()),
            |(head_pos, tail_pos): &mut (Pos, Pos), (direction, distance)| {
                let new_head_pos = head_pos.after_move(direction, distance);
                Some(new_head_pos)
            },
        )
        .unique()
        .count();
    Ok(())
}
