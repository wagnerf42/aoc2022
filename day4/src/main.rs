use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

use itertools::Itertools;
use regex::Regex;

fn contains_range<T: std::cmp::PartialOrd>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> bool {
    r1.contains(r2.start()) && r1.contains(r2.end())
}

fn touching<T: std::cmp::PartialOrd>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> bool {
    r1.contains(r2.start()) || r1.contains(r2.end())
}

fn overlaps<T: std::cmp::PartialOrd>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> bool {
    touching(r1, r2) || touching(r2, r1)
}

fn main() -> std::io::Result<()> {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    let c = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| {
            re.captures(&l)
                .unwrap()
                .iter()
                .flatten()
                .filter_map(|tok| tok.as_str().parse::<u32>().ok())
                .tuples()
                .map(|(start, end)| start..=end)
                .tuples()
                .next()
        })
        .filter(|(r1, r2)| contains_range(r1, r2) || contains_range(r2, r1))
        .count();
    eprintln!("c: {}", c);

    let c = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| {
            re.captures(&l)
                .unwrap()
                .iter()
                .flatten()
                .filter_map(|tok| tok.as_str().parse::<u32>().ok())
                .tuples()
                .map(|(start, end)| start..=end)
                .tuples()
                .next()
        })
        .filter(|(r1, r2)| overlaps(r1, r2))
        .count();
    eprintln!("c: {}", c);

    Ok(())
}
