use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
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
            let rate = captures.get(2).unwrap().as_str().parse::<u16>().unwrap();
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
    let mut reversed_nodes = nodes
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, n)| (n, i as u8))
        .collect::<HashMap<_, _>>();

    let mut rates = nodes
        .iter()
        .filter_map(|n| rates.get(n).cloned())
        .collect::<Vec<u16>>();

    // put all non zero rates first so that we can encode what is released in a u8
    assert!(rates.iter().filter(|&r| *r > 0).count() <= 16);
    let mut last_strong_rate = 0;
    for i in 0..nodes.len() {
        if rates[i] > 0 {
            nodes.swap(last_strong_rate, i);
            rates.swap(last_strong_rate, i);
            reversed_nodes.insert(nodes[i].clone(), i as u8);
            reversed_nodes.insert(nodes[last_strong_rate].clone(), last_strong_rate as u8);
            last_strong_rate += 1;
        }
    }

    let graph: Vec<Vec<u8>> = nodes
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

    {
        let mut writer = BufWriter::new(File::create("test.dot")?);
        writeln!(&mut writer, "digraph g {{")?;
        rates
            .iter()
            .enumerate()
            .filter_map(|(i, r)| if *r == 0 { None } else { Some(i) })
            .try_for_each(|i| writeln!(&mut writer, "n{} [color=\"red\"];", i))?;
        graph
            .iter()
            .enumerate()
            .flat_map(|(s, v)| v.iter().map(move |d| (s, d)))
            .try_for_each(|(s, d)| writeln!(&mut writer, "n{} -> n{};", s, d))?;
        writeln!(&mut writer, "}}")?;
    }

    let mut cache = HashMap::new();
    let max_release = solve(30, reversed_nodes["AA"], 0u16, &graph, &rates, &mut cache);

    println!("max is {max_release}");

    let mut cache = HashMap::with_capacity(1_800_000_000);
    let max_release = solve2(
        26,
        [reversed_nodes["AA"], reversed_nodes["AA"]],
        0u16,
        &graph,
        &rates,
        &mut cache,
    );

    println!("max is {max_release}");

    Ok(())
}

fn solve(
    minutes: u8,
    current_node: u8,
    released: u16,
    graph: &[Vec<u8>],
    rates: &[u16],
    cache: &mut HashMap<(u8, u8, u16), u16>,
) -> u16 {
    if minutes == 0 {
        return 0;
    }
    if let Some(answer) = cache.get(&(minutes, current_node, released)) {
        return *answer;
    }
    let best_value = if rates[current_node as usize] != 0
        && ((current_node < 16) && (released & (1 << current_node)) == 0)
    {
        Some(
            (minutes - 1) as u16 * rates[current_node as usize]
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
        graph[current_node as usize]
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
    current_nodes: [u8; 2],
    released: u16,
    graph: &[Vec<u8>],
    rates: &[u16],
    cache: &mut HashMap<(u8, [u8; 2], u16), u16>,
) -> u16 {
    if minutes == 0 {
        return 0;
    }
    if let Some(answer) = cache.get(&(minutes, current_nodes, released.clone())) {
        return *answer;
    }

    let evaluate = |c: [u8; 2]| {
        let mut value = 0;
        let mut new_released = released;
        for (i, n) in c.iter().enumerate() {
            if current_nodes[i] == *n && (*n < 16 && (new_released & 1 << *n) == 0) {
                value += rates[*n as usize] * (minutes as u16 - 1);
                new_released = new_released | (1 << *n);
            }
        }
        let v = if c[0] <= c[1] {
            [c[0], c[1]]
        } else {
            [c[1], c[0]]
        };
        value + solve2(minutes - 1, v, new_released, graph, rates, cache)
    };

    let next_configurations = |node: u8| {
        (rates[node as usize] != 0 && (released & (1 << node)) == 0)
            .then_some(node)
            .into_iter()
            .chain(graph[node as usize].iter().copied())
    };

    let best_value = if current_nodes[0] == current_nodes[1] {
        next_configurations(current_nodes[0])
            .enumerate()
            .flat_map(|(i, c1)| {
                next_configurations(current_nodes[0])
                    .skip(i + 1)
                    .map(move |c2| [c1, c2])
            })
            .map(evaluate)
            .max()
            .unwrap_or_else(|| {
                if (released & (1 << current_nodes[0])) == 0 {
                    rates[current_nodes[0] as usize] * (minutes as u16 - 1)
                } else {
                    0
                }
            })
    } else {
        next_configurations(current_nodes[0])
            .flat_map(|c1| next_configurations(current_nodes[1]).map(move |c2| [c1, c2]))
            .map(evaluate)
            .max()
            .unwrap()
    };
    cache.insert((minutes, current_nodes, released), best_value);
    best_value
}
