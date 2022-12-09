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
    fn pull_tail(&self, tail: &Self) -> Self {
        let x_sum = self.x + tail.x;
        let new_x = if x_sum % 2 == 0 { x_sum / 2 } else { self.x };

        let y_sum = self.y + tail.y;
        let new_y = if y_sum % 2 == 0 { y_sum / 2 } else { self.y };
        Pos { x: new_x, y: new_y }
    }
}

fn main() -> std::io::Result<()> {
    let tail_positions = BufReader::new(File::open("input")?)
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
        .flat_map(|(direction, distance)| std::iter::repeat(direction).take(distance as usize))
        .scan(
            (Default::default(), Default::default()),
            |(head, tail): &mut (Pos, Pos), direction| {
                let new_head = head.after_move(direction, 1);
                if new_head.distance(&tail) > 1 {
                    let new_tail = head.pull_tail(&tail);
                    *tail = new_tail;
                }
                *head = new_head;
                Some(*tail)
            },
        )
        .unique()
        .count();
    println!("tail positions: {:?}", tail_positions);

    let tail_positions = BufReader::new(File::open("input")?)
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
        .flat_map(|(direction, distance)| std::iter::repeat(direction).take(distance as usize))
        .scan(
            vec![Default::default(); 10],
            |knots: &mut Vec<Pos>, direction| {
                let head = knots.first_mut().unwrap();
                *head = head.after_move(direction, 1);
                for i in 1..10 {
                    if knots[i - 1].distance(&knots[i]) > 1 {
                        let moved_knot = knots[i - 1].pull_tail(&knots[i]);
                        knots[i] = moved_knot;
                    } else {
                        break;
                    }
                }
                knots.last().copied()
            },
        )
        .unique()
        .count();
    println!("tail positions: {:?}", tail_positions);

    Ok(())
}
