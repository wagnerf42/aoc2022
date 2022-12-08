use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

fn previous_max<'a, I: Iterator<Item = Pos> + 'a>(
    grid: &'a [Vec<i8>],
    iter: I,
) -> impl Iterator<Item = (i8, Pos, i8)> + 'a {
    iter.scan(-1, |max, pos| {
        let d = grid[pos.y][pos.x];
        let previous_max = *max;
        *max = previous_max.max(d);
        Some((previous_max, pos, d))
    })
    .take_while(|&(m, _, _)| m < 9)
}

fn previous_limit<'a, I: Iterator<Item = Pos> + 'a>(
    grid: &'a [Vec<i8>],
    iter: I,
) -> impl Iterator<Item = (Pos, Pos)> + 'a {
    let mut i = iter.peekable();
    let first_pos = i.peek().copied().unwrap();
    i.scan(vec![(10, first_pos)], |previous_highers, pos| {
        let d = grid[pos.y][pos.x];
        let mut sightblocking = None;
        while previous_highers
            .last()
            .map(|&(height, _)| height <= d)
            .unwrap()
        {
            let (height, previous_pos) = previous_highers.pop().unwrap();
            if height == d {
                sightblocking = Some(previous_pos)
            }
        }
        if sightblocking.is_none() {
            sightblocking = previous_highers.last().map(|&(_, pos)| pos);
        }
        previous_highers.push((d, pos));
        Some((sightblocking.unwrap(), pos))
    })
}

impl Pos {
    fn distance(&self, other: &Self) -> usize {
        let xmin = self.x.min(other.x);
        let xmax = self.x.max(other.x);
        let ymin = self.y.min(other.y);
        let ymax = self.y.max(other.y);
        (xmax - xmin) + (ymax - ymin)
    }
}

fn main() -> std::io::Result<()> {
    let grid = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as i8))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let lines = grid.len();
    let columns = grid.get(0).map(|l| l.len()).unwrap_or_default();

    let mut left_right =
        (0..lines).flat_map(|y| previous_max(&grid, (0..columns).map(move |x| Pos { x, y })));
    let mut top_bottom =
        (0..columns).flat_map(|x| previous_max(&grid, (0..lines).map(move |y| Pos { x, y })));
    let mut right_left =
        (0..lines).flat_map(|y| previous_max(&grid, (0..columns).map(move |x| Pos { x, y }).rev()));
    let mut bottom_top =
        (0..columns).flat_map(|x| previous_max(&grid, (0..lines).map(move |y| Pos { x, y }).rev()));

    let mut iterators: [&mut dyn Iterator<Item = (i8, Pos, i8)>; 4] = [
        &mut left_right,
        &mut top_bottom,
        &mut right_left,
        &mut bottom_top,
    ];

    let visible = iterators
        .iter_mut()
        .flatten()
        .filter(|&(max, _, d)| max < d)
        .map(|(_, p, _)| p)
        .unique()
        .count();

    println!("visible {}", visible);

    let mut left_right =
        (0..lines).flat_map(|y| previous_limit(&grid, (0..columns).map(move |x| Pos { x, y })));
    let mut top_bottom =
        (0..columns).flat_map(|x| previous_limit(&grid, (0..lines).map(move |y| Pos { x, y })));
    let mut right_left = (0..lines)
        .flat_map(|y| previous_limit(&grid, (0..columns).map(move |x| Pos { x, y }).rev()));
    let mut bottom_top = (0..columns)
        .flat_map(|x| previous_limit(&grid, (0..lines).map(move |y| Pos { x, y }).rev()));

    let mut iterators: [&mut dyn Iterator<Item = (Pos, Pos)>; 4] = [
        &mut left_right,
        &mut top_bottom,
        &mut right_left,
        &mut bottom_top,
    ];

    let scores = iterators
        .iter_mut()
        .flatten()
        .map(|(previous_pos, pos)| (previous_pos.distance(&pos), pos))
        .fold(HashMap::new(), |mut scores, (distance, pos)| {
            *scores.entry(pos).or_insert(1) *= distance;
            scores
        });

    let best_score = scores.values().copied().max().unwrap_or_default();

    println!("best scenic score {}", best_score);

    Ok(())
}
