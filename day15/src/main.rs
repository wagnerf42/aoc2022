use itertools::Itertools;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn update_ranges(starts: &[i32], ends: &[i32], ydiff: i32) -> (Vec<i32>, Vec<i32>) {
    starts
        .iter()
        .zip(ends.iter())
        .filter(|&(s, e)| e - s > 2 * ydiff)
        .map(|(s, e)| (s + ydiff, e - ydiff))
        .unzip()
}

fn count_inside(mut starts: Vec<i32>, mut ends: Vec<i32>) -> i32 {
    starts.sort_unstable();
    ends.sort_unstable();
    starts
        .iter()
        .map(|s| (s, 1))
        .merge(ends.iter().map(|e| (e, -1)))
        .scan((0, None), |(inside, last_start), (x, insidediff)| {
            *inside += insidediff;
            if *inside == 0 {
                Some(last_start.take().map(|s| (s, x)))
            } else if *inside == 1 && insidediff == 1 {
                // we start
                *last_start = Some(x);
                Some(None)
            } else {
                Some(None)
            }
        })
        .flatten()
        .map(|(s, e)| e - s)
        .sum()
}

const TARGET: i32 = 2_000_000;

fn main() -> std::io::Result<()> {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    //TODO: i forgot that sensors below also affect above
    //TODO: also we need to count the '#' but not the 'B'
    let mut beacons: HashMap<i32, Vec<i32>> = HashMap::new();
    let (starts, ends, y) = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| {
            re.captures(&l)
                .unwrap()
                .iter()
                .flatten()
                .filter_map(|tok| tok.as_str().parse::<i32>().ok())
                .tuples()
                .next()
        })
        .sorted_by_key(|(_, sy, _, _)| *sy)
        .inspect(|(_, _, bx, by)| beacons.entry(*by).or_default().push(*bx))
        .map(|(sx, sy, bx, by)| {
            let distance = (bx - sx).abs() + (by - sy).abs();
            ((sx - distance), (sx + distance + 1), sy)
        })
        .tuple_windows()
        .scan(
            (Vec::new(), Vec::new()),
            |(starts, ends), ((start, end, y), (_, _, next_y))| {
                let ydiff = next_y - y;
                let (mut new_starts, mut new_ends) = update_ranges(starts, ends, ydiff);
                new_starts.push(start);
                new_ends.push(end);
                std::mem::swap(starts, &mut new_starts);
                std::mem::swap(ends, &mut new_ends);
                Some((new_starts, new_ends, y))
            },
        )
        .take_while(|&(_, _, y)| y <= TARGET)
        .last()
        .unwrap();
    let (starts, ends) = update_ranges(&starts, &ends, TARGET - y);
    let r = count_inside(starts, ends);
    eprintln!("{r}");
    Ok(())
}
