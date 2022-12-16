use itertools::Itertools;
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

    assert!(nodes.len() <= 64);

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

    let mut cache = HashMap::new();
    let max_release = solve(30, reversed_nodes["AA"], 0u64, &graph, &rates, &mut cache);

    println!("max is {max_release}");

    let mut cache = HashMap::new();
    let max_release = solve2(
        26,
        [reversed_nodes["AA"], reversed_nodes["AA"]],
        0u64,
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
    released: u64,
    graph: &[Vec<usize>],
    rates: &[u32],
    cache: &mut HashMap<(u8, usize, u64), u32>,
) -> u32 {
    if minutes == 0 {
        return 0;
    }
    if let Some(answer) = cache.get(&(minutes, current_node, released)) {
        return *answer;
    }
    let best_value = if rates[current_node] != 0 && (released & (1 << current_node)) == 0 {
        Some(
            (minutes - 1) as u32 * rates[current_node]
                + solve(
                    minutes - 1,
                    current_node,
                    released | (1 << current_node),
                    graph,
                    rates,
                    cache,
                ),
        )
    } else {
        None
    }
    .into_iter()
    .chain(
        graph[current_node]
            .iter()
            .map(|n| solve(minutes - 1, *n, released, graph, rates, cache)),
    )
    .max()
    .unwrap();
    cache.insert((minutes, current_node, released), best_value);
    best_value
}

fn solve2(
    minutes: u8,
    current_nodes: [usize; 2],
    released: u64,
    graph: &[Vec<usize>],
    rates: &[u32],
    cache: &mut HashMap<(u8, [usize; 2], u64), u32>,
) -> u32 {
    if minutes == 0 {
        return 0;
    }
    if let Some(answer) = cache.get(&(minutes, current_nodes, released.clone())) {
        return *answer;
    }
    let best_value = current_nodes
        .iter()
        .map(|node| {
            (rates[*node] != 0 && (released & (1 << *node)) == 0)
                .then_some((*node, true))
                .into_iter()
                .chain(graph[*node].iter().map(|n| (*n, false)))
        })
        .multi_cartesian_product()
        .filter_map(|mut v| {
            v.sort_unstable();
            (v[0] != v[1] || !v[0].1).then_some(v) // let's not release twice the same
        })
        .map(|c| {
            let mut value = 0;
            let mut new_released = released;
            for (n, release) in c.iter() {
                if *release {
                    value += rates[*n] * (minutes as u32 - 1);
                    new_released = released | (1 << *n);
                }
            }
            value
                + solve2(
                    minutes - 1,
                    [c[0].0, c[1].0],
                    new_released,
                    graph,
                    rates,
                    cache,
                )
        })
        .max()
        .unwrap();
    cache.insert((minutes, current_nodes, released), best_value);
    best_value
}
