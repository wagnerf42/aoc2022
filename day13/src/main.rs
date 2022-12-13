use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;
use json::JsonValue;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tree {
    Integer(u64),
    List(Vec<Tree>),
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Tree::Integer(iself) => match other {
                Tree::Integer(iother) => iself.partial_cmp(iother),
                Tree::List(lother) => std::iter::once(self).partial_cmp(lother.iter()),
            },
            Tree::List(lself) => match other {
                Tree::Integer(_) => lself.iter().partial_cmp(std::iter::once(other)),
                Tree::List(lother) => lself.iter().partial_cmp(lother.iter()),
            },
        }
    }
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_tree(content: &str) -> Tree {
    convert(&json::parse(content).unwrap())
}

fn convert(jval: &JsonValue) -> Tree {
    match jval {
        JsonValue::Number(n) => Tree::Integer(n.as_fixed_point_u64(0).unwrap()),
        JsonValue::Array(v) => Tree::List(v.iter().map(convert).collect()),
        _ => panic!(),
    }
}

fn main() -> std::io::Result<()> {
    let count = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.is_empty())
        .map(|l| parse_tree(&l))
        .tuples()
        .enumerate()
        .filter_map(|(i, (t1, t2))| if t1 < t2 { Some(i + 1) } else { None })
        .sum::<usize>();
    println!("{count}");

    let divider1 = parse_tree("[[2]]");
    let divider2 = parse_tree("[[6]]");
    let mut trees = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.is_empty())
        .map(|l| parse_tree(&l))
        .chain([divider1.clone(), divider2.clone()])
        .collect::<Vec<_>>();
    trees.sort_unstable();
    let p = trees
        .iter()
        .enumerate()
        .filter_map(|(i, t)| {
            if t == &divider1 || t == &divider2 {
                Some(i + 1)
            } else {
                None
            }
        })
        .product::<usize>();
    println!("{p}");
    Ok(())
}
