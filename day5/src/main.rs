use itertools::Itertools;
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn load_crates(lines: impl Iterator<Item = String>) -> std::io::Result<Vec<VecDeque<char>>> {
    let mut crates = Vec::new();
    for line in lines.take_while(|l| !l.is_empty()) {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            while crates.len() < i + 1 {
                crates.push(VecDeque::new());
            }
            if !c.is_whitespace() && !c.is_ascii_digit() {
                crates[i].push_front(c);
            }
        }
    }
    Ok(crates)
}

fn borrow_mut<T>(s: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    if i < j {
        let (start, end) = s.split_at_mut(j);
        (&mut start[i], &mut end[0])
    } else {
        let (start, end) = s.split_at_mut(i);
        (&mut end[0], &mut start[j])
    }
}

fn main() -> std::io::Result<()> {
    let read = BufReader::new(File::open("input")?);
    let mut lines = read.lines().filter_map(|l| l.ok());
    let mut crates = load_crates(&mut lines)?;
    for command_line in lines {
        for (amount, source, target) in command_line
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .filter_map(|t| t.parse::<usize>().ok())
            .tuples()
        {
            for _ in 0..amount {
                let c = crates[source - 1].pop_back().unwrap();
                crates[target - 1].push_back(c);
            }
        }
    }
    let msg = crates.iter().map(|c| c.back().unwrap()).collect::<String>();
    println!("message is {}", msg);

    let read = BufReader::new(File::open("input")?);
    let mut lines = read.lines().filter_map(|l| l.ok());
    let mut crates = load_crates(&mut lines)?;
    for command_line in lines {
        for (amount, source, target) in command_line
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .filter_map(|t| t.parse::<usize>().ok())
            .tuples()
        {
            let (source_crate, target_crate) = borrow_mut(&mut crates, source - 1, target - 1);
            let n = source_crate.len();
            target_crate.extend(source_crate.drain((n - amount)..));
        }
    }
    let msg = crates.iter().map(|c| c.back().unwrap()).collect::<String>();
    println!("message is {}", msg);

    Ok(())
}
