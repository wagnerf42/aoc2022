use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let s = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| {
            let mut tokens = l.split_whitespace();
            let cmd = tokens.next().unwrap();
            if cmd == "noop" {
                None
            } else {
                tokens.next().and_then(|t| t.parse::<i32>().ok())
            }
        })
        .scan(1, |value, cmd| {
            if let Some(added) = cmd {
                let old_value = *value;
                *value += added;
                Some(Some(old_value).into_iter().chain(Some(old_value)))
            } else {
                Some(Some(*value).into_iter().chain(None))
            }
        })
        .flatten()
        .skip(19)
        .step_by(40)
        .enumerate()
        .map(|(i, x)| (20 + i as i32 * 40) * x)
        .sum::<i32>();
    println!("sum {}", s);

    let screen = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| {
            let mut tokens = l.split_whitespace();
            let cmd = tokens.next().unwrap();
            if cmd == "noop" {
                None
            } else {
                tokens.next().and_then(|t| t.parse::<i32>().ok())
            }
        })
        .scan(1, |value, cmd| {
            if let Some(added) = cmd {
                let old_value = *value;
                *value += added;
                Some(Some(old_value).into_iter().chain(Some(old_value)))
            } else {
                Some(Some(*value).into_iter().chain(None))
            }
        })
        .flatten()
        .enumerate()
        .map(|(i, p)| (i % 40, p))
        .map(|(i, p)| {
            if ((p - 1)..=(p + 1)).contains(&(i as i32)) {
                '#'
            } else {
                '.'
            }
        })
        .chunks(40)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .join("\n");
    println!("{screen}");

    Ok(())
}
