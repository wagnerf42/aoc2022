use bitvec::prelude::*;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    // parse graph
    let re = Regex::new(
        r"Valve (\S+) has flow rate=(\d+); tunnels? leads? to valves? (\S+(?:,\s+\S+)*)$",
    )
    .unwrap();

    let mut nodes: Vec<String> = Vec::new();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut rates = HashMap::new();
    BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .for_each(|l| {
            let captures = re.captures(&l).unwrap();
            let node_id = captures.get(1).unwrap().as_str().to_string();
            nodes.push(node_id.clone());
            let rate = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let neighbours = captures
                .get(3)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|n| n.to_string());
            graph.entry(node_id.clone()).or_default().extend(neighbours);
            rates.insert(node_id, rate);
        });

    // remove all strings
    let reversed_nodes = nodes
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect::<HashMap<_, _>>();

    let graph: Vec<Vec<usize>> = nodes
        .iter()
        .map(|id| {
            graph
                .get(id)
                .map(|n| n.iter().filter_map(|n| reversed_nodes.get(n).cloned()))
                .into_iter()
                .flatten()
                .collect()
        })
        .collect::<Vec<_>>();

    let rates = nodes
        .iter()
        .filter_map(|n| rates.get(n).cloned())
        .collect::<Vec<u32>>();
    let mut released = bitvec![u64, Msb0;];
    released.extend(std::iter::repeat(false).take(nodes.len()));

    let mut cache = HashMap::new();
    let max_release = solve(
        30,
        *reversed_nodes.get("AA").unwrap(),
        released,
        &graph,
        &rates,
        &mut cache,
    );

    println!("max is {max_release}");
    Ok(())
}

fn solve(
    minutes: u8,
    current_node: usize,
    released: BitVec<u64, Msb0>,
    graph: &[Vec<usize>],
    rates: &[u32],
    cache: &mut HashMap<(u8, usize, BitVec<u64, Msb0>), u32>,
) -> u32 {
    if minutes == 0 {
        return 0;
    }
    if let Some(answer) = cache.get(&(minutes, current_node, released.clone())) {
        return *answer;
    }
    let best_value = if rates[current_node] != 0 && !released[current_node] {
        let mut releasing = released.clone();
        releasing.set(current_node, true);
        Some(
            (minutes - 1) as u32 * rates[current_node]
                + solve(minutes - 1, current_node, releasing, graph, rates, cache),
        )
    } else {
        None
    }
    .into_iter()
    .chain(
        graph[current_node]
            .iter()
            .map(|n| solve(minutes - 1, *n, released.clone(), graph, rates, cache)),
    )
    .max()
    .unwrap();
    cache.insert((minutes, current_node, released), best_value);
    best_value
}
