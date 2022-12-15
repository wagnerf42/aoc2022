use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
};

//well, that was awful

fn count_inside(mut starts: Vec<i32>, mut ends: Vec<i32>, beacons: Option<&HashSet<i32>>) -> i32 {
    starts.sort_unstable();
    ends.sort_unstable();
    let mut inner_beacons = 0;
    starts
        .iter()
        .map(|s| (s, 1, false))
        .merge(ends.iter().map(|e| (e, -1, false)))
        .merge(beacons.into_iter().flatten().sorted().map(|x| (x, 0, true)))
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

// loop on full_range minus the union of the ranges (part2)
fn minus_union<I: Iterator<Item = Range<i32>>>(
    full_range: Range<i32>,
    ranges: I,
) -> impl Iterator<Item = Range<i32>> {
    let mut u = union(ranges)
        .take_while(move |r| r.start <= full_range.end)
        .map(move |r| r.start.max(full_range.start)..(r.end.min(full_range.end)))
        .peekable();
    let start = u.peek().cloned().or(Some(full_range.end..full_range.end));
    start.map(|s| full_range.start..s.start).into_iter().chain(
        u.chain(std::iter::once(full_range.end..full_range.end))
            .tuple_windows()
            .map(|(r1, r2)| r1.end..r2.start),
    )
}

// loop on the union of given ranges (part2)
fn union<I: Iterator<Item = Range<i32>>>(ranges: I) -> impl Iterator<Item = Range<i32>> {
    let (mut starts, mut ends): (Vec<i32>, Vec<i32>) = ranges.map(|r| (r.start, r.end)).unzip();
    starts.sort_unstable();
    ends.sort_unstable();

    starts
        .into_iter()
        .map(|s| (s, 1))
        .merge(ends.into_iter().map(|e| (e, -1)))
        .scan((0, None), |(inside, last_start), (x, insidediff)| {
            *inside += insidediff;
            if *inside == 0 {
                Some(last_start.take().map(|s| s..x))
            } else if *inside == 1 && insidediff == 1 {
                // we start
                *last_start = Some(x);
                Some(None)
            } else {
                Some(None)
            }
        })
        .flatten()
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
    println!("{r}");

    // part 2

    let ranges = BufReader::new(File::open("input")?)
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
        .map(|(sx, sy, bx, by)| {
            let distance = (bx - sx).abs() + (by - sy).abs();
            let range_start = sx - distance;
            let range_end = sx + distance + 1;
            (range_start, range_end, sy)
        })
        .collect::<Vec<_>>();

    let f = (0..4_000_001)
        .find_map(|y| {
            minus_union(
                0..4_000_001,
                ranges.iter().filter_map(|(sx, ex, yr)| {
                    let d = (yr - y).abs();
                    let r = (sx + d)..(ex - d);
                    if r.is_empty() {
                        None
                    } else {
                        Some(r)
                    }
                }),
            )
            .find(|r| !r.is_empty())
            .map(|r| {
                assert!(r.start == r.end - 1);
                r.start as u64 * 4_000_000 + y as u64
            })
        })
        .unwrap();

    eprintln!("tuning frequency : {f}");

    Ok(())
}
