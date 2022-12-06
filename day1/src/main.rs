use itertools::Itertools;
use std::{
    cmp::Reverse,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let calories: u32 = BufReader::new(File::open("input1")?)
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| l.parse::<u32>().ok())
        .batching(|i| i.while_some().sum1())
        .max()
        .unwrap_or_default();
    eprintln!("max {}", calories);

    let calories: u32 = BufReader::new(File::open("input1")?)
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| l.parse::<u32>().ok())
        .batching(|i| i.while_some().sum1::<u32>())
        .map(Reverse)
        .k_smallest(3)
        .into_iter()
        .map(|s| s.0)
        .sum();
    println!("calories: {}", calories);

    Ok(())
}
