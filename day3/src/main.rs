use itertools::Itertools;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let s: u32 = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .map(|line| {
            let n = line.len();
            let (left, right) = line.split_at(n / 2);
            let left_chars = left.chars().collect::<HashSet<_>>();
            right.chars().find(|c| left_chars.contains(c)).unwrap()
        })
        .map(|c| {
            let i = c as u8;
            (1 + if c.is_ascii_uppercase() {
                i + 26 - b'A'
            } else {
                i - b'a'
            }) as u32
        })
        .sum();
    println!("sum: {}", s);

    let s: u32 = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .tuples()
        .map(|(r1, r2, r3)| {
            let c1 = r1.chars().collect::<HashSet<_>>();
            let c2 = r2.chars().collect::<HashSet<_>>();
            let i = c1.intersection(&c2).collect::<HashSet<_>>();
            r3.chars().find(|c| i.contains(c)).unwrap()
        })
        .map(|c| {
            let i = c as u8;
            (1 + if c.is_ascii_uppercase() {
                i + 26 - b'A'
            } else {
                i - b'a'
            }) as u32
        })
        .sum();
    println!("sum: {}", s);

    Ok(())
}
