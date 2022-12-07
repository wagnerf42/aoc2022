use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

enum Entry {
    File(u32, String),
    Dir(String),
}

struct Dir {
    absolute_name: String,
    entries: Vec<Entry>,
}

enum InputLine {
    Cd(String),
    Entry(Entry),
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let start = tokens.next().ok_or(())?;
        if start == "dir" {
            tokens
                .next()
                .ok_or(())
                .map(|dirname| Entry::Dir(dirname.to_string()))
        } else {
            start.parse::<u32>().map_err(|_| ()).and_then(|size| {
                tokens
                    .next()
                    .ok_or(())
                    .map(|filename| Entry::File(size, filename.to_string()))
            })
        }
    }
}

impl FromStr for InputLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("$ cd") {
            Ok(InputLine::Cd(s[5..].to_string()))
        } else if s.starts_with("$ ls") {
            Err(())
        } else {
            s.parse::<Entry>().map(InputLine::Entry)
        }
    }
}

fn main() -> std::io::Result<()> {
    BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<InputLine>().ok());
    Ok(())
}
