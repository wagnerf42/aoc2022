use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

use fraction::Fraction;
use itertools::Itertools;

fn main() -> std::io::Result<()> {
    let numbers = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<i32>().ok())
        .collect::<Vec<_>>();
    let mut positions = numbers
        .iter()
        .copied()
        .enumerate()
        .map(|(i, n)| (Fraction::from(i), n))
        .collect::<BTreeMap<_, _>>();
    let mut zero_pos = None;
    for i in 0..numbers.len() {
        let current_pos = Fraction::from(i);
        let number = *positions.get(&current_pos).unwrap();
        if number == 0 {
            zero_pos = Some(current_pos);
            continue;
        }

        positions.remove(&current_pos);
        let new_pos = if number > 0 {
            positions
                .range(current_pos..)
                .chain(positions.range(..current_pos))
                .cycle()
                .map(|(f, _)| *f)
                .tuple_windows()
                .nth((number - 1) as usize)
                .map(|(end, after_end)| {
                    if after_end > end {
                        (end + after_end) / 2
                    } else {
                        end + 1
                    }
                })
                .unwrap()
        } else {
            positions
                .range(current_pos..)
                .chain(positions.range(..current_pos))
                .rev()
                .cycle()
                .map(|(f, _)| *f)
                .tuple_windows()
                .nth(number.unsigned_abs() as usize - 1)
                .map(|(end, after_end)| {
                    if after_end < end {
                        (end + after_end) / 2
                    } else {
                        end - 1
                    }
                })
                .unwrap()
        };
        positions.insert(new_pos, number);
    }

    let zero_pos = zero_pos.unwrap();
    let s = positions
        .range(zero_pos..)
        .chain(positions.range(..zero_pos))
        .cycle()
        .map(|(_, n)| n)
        .step_by(1_000)
        .take(4)
        .sum::<i32>();
    println!("s: {s}");
    Ok(())
}
