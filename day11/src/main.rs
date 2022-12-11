use std::{
    cmp::Reverse,
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

#[derive(Debug)]
enum Op {
    Add(u64),
    Mul(u64),
    Square,
}

impl Op {
    fn apply(&self, val: u64) -> u64 {
        match self {
            Op::Add(e) => val + e,
            Op::Mul(e) => val * e,
            Op::Square => val * val,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    op: Op,
    modulo: u64,
    throws: [usize; 2],
}

impl Monkey {
    fn inspect(&self, item: u64) -> u64 {
        self.op.apply(item)
    }
}

fn last_int(line: &str) -> u64 {
    line.split_whitespace()
        .last()
        .and_then(|t| t.parse::<u64>().ok())
        .unwrap()
}

fn main() -> std::io::Result<()> {
    let mut items = Vec::new();
    let monkeys: Vec<_> = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.is_empty())
        .tuples()
        .map(|(_, monkey_items, op, test, throw_true, throw_false)| {
            items.push(
                monkey_items
                    .split_whitespace()
                    .skip(2)
                    .map(|t| t.strip_suffix(',').unwrap_or(t).parse::<u64>().unwrap())
                    .collect::<VecDeque<_>>(),
            );
            let mut op_tokens = op.split_whitespace().skip(4);
            let op_type = op_tokens.next().unwrap();
            let op = op_tokens
                .next()
                .unwrap()
                .parse::<u64>()
                .map(|num| if op_type == "+" { Op::Add } else { Op::Mul }(num))
                .unwrap_or(Op::Square);
            Monkey {
                op,
                modulo: last_int(&test),
                throws: [
                    last_int(&throw_false) as usize,
                    last_int(&throw_true) as usize,
                ],
            }
        })
        .collect();
    let backup_items = items.clone(); // for part 2;

    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..20 {
        for (index, monkey) in monkeys.iter().enumerate() {
            while let Some(item) = items[index].pop_front() {
                inspections[index] += 1;
                let new_item = monkey.inspect(item) / 3;

                let target_monkey_index =
                    monkey.throws[if new_item % monkey.modulo == 0 { 1 } else { 0 }];
                items[target_monkey_index].push_back(new_item);
            }
        }
    }
    let business = inspections
        .iter()
        .map(Reverse)
        .k_smallest(2)
        .map(|r| r.0)
        .product::<u64>();
    println!("{business}");

    let mut inspections = vec![0; monkeys.len()];
    let mut items = backup_items;
    let modulo_prod = monkeys.iter().map(|m| m.modulo).product::<u64>();
    for _ in 0..10_000 {
        for (index, monkey) in monkeys.iter().enumerate() {
            while let Some(item) = items[index].pop_front() {
                inspections[index] += 1;
                let new_item = monkey.inspect(item) % modulo_prod;

                let target_monkey_index =
                    monkey.throws[if new_item % monkey.modulo == 0 { 1 } else { 0 }];
                items[target_monkey_index].push_back(new_item);
            }
        }
    }
    let business = inspections
        .iter()
        .map(Reverse)
        .k_smallest(2)
        .map(|r| r.0)
        .product::<u64>();
    println!("{business}");

    Ok(())
}
