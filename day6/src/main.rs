use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufReader, Read},
};

fn main() -> std::io::Result<()> {
    let mut bytes = Vec::new();
    BufReader::new(File::open("input")?).read_to_end(&mut bytes)?;

    let pos = bytes
        .windows(4)
        .enumerate()
        .find_map(|(i, w)| {
            if w.iter().unique().count() == 4 {
                Some(i)
            } else {
                None
            }
        })
        .unwrap()
        + 4;
    println!("position is {}", pos);

    let mut bytes = BufReader::new(File::open("input")?)
        .bytes()
        .filter_map(|b| b.ok());

    let windows = (&mut bytes).take(14).collect::<VecDeque<_>>();
    let counts: HashMap<u8, usize> = windows.iter().fold(HashMap::new(), |mut h, b| {
        *h.entry(*b).or_default() += 1;
        h
    });

    let pos = if counts.len() == 14 {
        14
    } else {
        bytes
            .scan((windows, counts), |(windows, counts), b| {
                windows.push_back(b);
                let old = windows.pop_front().unwrap();
                *counts.entry(b).or_default() += 1;
                let old_count = counts.get_mut(&old).unwrap();
                *old_count -= 1;
                if *old_count == 0 {
                    counts.remove(&old);
                }
                Some(counts.len())
            })
            .enumerate()
            .find_map(|(i, l)| if l == 14 { Some(i) } else { None })
            .unwrap()
            + 15
    };
    println!("position is {}", pos);

    Ok(())
}
