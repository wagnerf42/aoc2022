use itertools::Itertools;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const MATCH_POINTS: [[u8; 3]; 3] = [[3, 0, 6], [6, 3, 0], [0, 6, 3]];
const NEEDS_LOSING: [u8; 3] = [2, 0, 1];
const NEEDS_WINNING: [u8; 3] = [1, 2, 0];

fn parse(code: &str) -> u8 {
    match code {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => panic!(),
    }
}

fn main() -> std::io::Result<()> {
    let score: u32 = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.split_whitespace().map(parse).tuples().next())
        .map(|(him, me)| (MATCH_POINTS[me as usize][him as usize] + me + 1) as u32)
        .sum();
    println!("score {}", score);

    let score: u32 = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.split_whitespace().map(parse).tuples().next())
        .map(|(him, needed)| match needed {
            0 => NEEDS_LOSING[him as usize] + 1,
            1 => him + 1 + 3,
            2 => NEEDS_WINNING[him as usize] + 1 + 6,
            _ => unreachable!(),
        } as u32)
        .sum();
    println!("score {}", score);
    Ok(())
}
