use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn count_inside(mut starts: Vec<i32>, mut ends: Vec<i32>, beacons: Option<&HashSet<i32>>) -> i32 {
    starts.sort_unstable();
    ends.sort_unstable();
    let mut inner_beacons = 0;
    starts
        .iter()
        .map(|s| (s, 1, false))
        .merge(ends.iter().map(|e| (e, -1, false)))
        .merge(
            beacons
                .map(|v| &*v)
                .into_iter()
                .flatten()
                .sorted()
                .map(|x| (x, 0, true)),
        )
        .scan(
            (0, None),
            |(inside, last_start), (x, insidediff, is_beacon)| {
                if is_beacon {
                    if *inside >= 1 {
                        inner_beacons += 1;
                    }
                    Some(None)
                } else {
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
                }
            },
        )
        .flatten()
        .map(|(s, e)| e - s)
        .sum::<i32>()
        - inner_beacons
}

const TARGET: i32 = 2_000_000;

fn main() -> std::io::Result<()> {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    let mut beacons: HashMap<i32, HashSet<i32>> = HashMap::new();
    let (starts, ends): (Vec<i32>, Vec<i32>) = BufReader::new(File::open("input")?)
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
        .inspect(|(_, _, bx, by)| {
            beacons.entry(*by).or_default().insert(*bx);
        })
        .filter_map(|(sx, sy, bx, by)| {
            let distance = (bx - sx).abs() + (by - sy).abs();
            let mut range_start = sx - distance;
            let mut range_end = sx + distance + 1;
            let lines_to_go = (TARGET - sy).abs();
            range_start += lines_to_go;
            range_end -= lines_to_go;
            if range_start < range_end {
                Some((range_start, range_end))
            } else {
                None
            }
        })
        .unzip();

    let r = count_inside(starts, ends, beacons.get(&TARGET));
    eprintln!("{r}");
    Ok(())
}
