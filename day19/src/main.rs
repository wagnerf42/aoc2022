use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

fn main() -> std::io::Result<()> {
    let quality = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| {
            l.split_whitespace()
                .filter_map(|t| t.parse::<u32>().ok())
                .tuples()
                .next()
                .map(|(o1, o2, o3, c1, o4, obsidian)| {
                    [[o1, 0, 0], [o2, 0, 0], [o3, c1, 0], [o4, 0, obsidian]]
                })
        })
        .map(|blueprint| solve(blueprint, 24))
        .enumerate()
        .map(|(i, geodes)| (i as u32 + 1) * geodes)
        .sum::<u32>();
    println!("{quality}");

    let product = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .take(3)
        .filter_map(|l| {
            l.split_whitespace()
                .filter_map(|t| t.parse::<u32>().ok())
                .tuples()
                .next()
                .map(|(o1, o2, o3, c1, o4, obsidian)| {
                    [[o1, 0, 0], [o2, 0, 0], [o3, c1, 0], [o4, 0, obsidian]]
                })
        })
        .map(|blueprint| solve(blueprint, 32))
        .product::<u32>();
    println!("{product}");

    Ok(())
}

fn solve(blueprint: [[u32; 3]; 4], minutes: u8) -> u32 {
    eprintln!("{blueprint:?}");
    let mut cache = HashMap::new();
    (0..2)
        .map(|target| {
            solve_rec(
                &blueprint,
                minutes,
                target,
                [1, 0, 0, 0],
                [0, 0, 0, 0],
                &mut cache,
            )
        })
        .max()
        .unwrap_or_default()
}

fn solve_rec(
    blueprint: &[[u32; 3]; 4],
    minutes: u8,
    target: u8,
    robots: [u32; 4],
    resources: [u32; 4],
    cache: &mut HashMap<(u8, u8, [u32; 4], [u32; 4]), u32>,
) -> u32 {
    if let Some(cached) = cache.get(&(minutes, target, robots, resources)) {
        return *cached;
    }
    let for_target = &blueprint[target as usize];
    let minutes_waiting = for_target
        .iter()
        .zip(&resources)
        .map(|(needed, available)| needed.saturating_sub(*available))
        .zip(&robots)
        .filter(|&(needed, _)| needed > 0)
        .map(|(needed, per_turn)| needed / per_turn + if needed % per_turn == 0 { 0 } else { 1 })
        .max()
        .unwrap_or_default()
        + 1; // one extra for build time

    let res = {
        if minutes_waiting >= minutes as u32 {
            resources.last().copied().unwrap() + robots.last().copied().unwrap() * minutes as u32
        } else {
            let mut new_resources = resources;
            new_resources
                .iter_mut()
                .zip(&robots)
                .for_each(|(res, rob)| *res += *rob * minutes_waiting);

            new_resources
                .iter_mut()
                .zip(&blueprint[target as usize])
                .for_each(|(res, req)| *res = res.checked_sub(*req).unwrap());
            let mut new_robots = robots;
            new_robots[target as usize] += 1;
            blueprint
                .iter()
                .enumerate()
                .filter(|&(_, print)| {
                    print
                        .iter()
                        .zip(&robots)
                        .all(|(req, rob)| *req == 0 || *rob > 0)
                })
                .map(|(new_target, _)| {
                    solve_rec(
                        blueprint,
                        minutes - minutes_waiting as u8,
                        new_target as u8,
                        new_robots,
                        new_resources,
                        cache,
                    )
                })
                .max()
                .unwrap_or_default()
        }
    };
    cache.insert((minutes, target, robots, resources), res);
    res
}
