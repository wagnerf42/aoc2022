use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn previous_max<'a, I: Iterator<Item = (usize, usize)> + 'a>(
    grid: &'a [Vec<i8>],
    iter: I,
) -> impl Iterator<Item = (i8, usize, usize, i8)> + 'a {
    iter.scan(-1, |max, (y, x)| {
        let d = grid[y][x];
        let previous_max = *max;
        *max = previous_max.max(d);
        Some((previous_max, y, x, d))
    })
    .take_while(|&(m, _, _, _)| m < 9)
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
        (0..lines).flat_map(|y| previous_max(&grid, (0..columns).map(move |x| (y, x))));
    let mut top_bottom =
        (0..columns).flat_map(|x| previous_max(&grid, (0..lines).map(move |y| (y, x))));
    let mut right_left =
        (0..lines).flat_map(|y| previous_max(&grid, (0..columns).map(move |x| (y, x)).rev()));
    let mut bottom_top =
        (0..columns).flat_map(|x| previous_max(&grid, (0..lines).map(move |y| (y, x)).rev()));

    let mut iterators: [&mut dyn Iterator<Item = (i8, usize, usize, i8)>; 4] = [
        &mut left_right,
        &mut top_bottom,
        &mut right_left,
        &mut bottom_top,
    ];

    let visible = iterators
        .iter_mut()
        .flatten()
        .filter(|&(max, _, _, d)| max < d)
        .map(|(_, y, x, _)| (y, x))
        .unique()
        .count();

    println!("visible {}", visible);

    Ok(())
}
