use std::{
    collections::HashSet,
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
                .filter_map(|t| t.parse::<u16>().ok())
                .tuples()
                .next()
                .map(|(o1, o2, o3, c1, o4, obsidian)| {
                    [[o1, 0, 0], [o2, 0, 0], [o3, c1, 0], [o4, 0, obsidian]]
                })
        })
        .map(|blueprint| solve(blueprint, 24))
        .enumerate()
        .map(|(i, geodes)| (i as u16 + 1) * geodes)
        .sum::<u16>();
    println!("{quality}");

    let product = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .take(3)
        .filter_map(|l| {
            l.split_whitespace()
                .filter_map(|t| t.parse::<u16>().ok())
                .tuples()
                .next()
                .map(|(o1, o2, o3, c1, o4, obsidian)| {
                    [[o1, 0, 0], [o2, 0, 0], [o3, c1, 0], [o4, 0, obsidian]]
                })
        })
        .map(|blueprint| solve(blueprint, 32))
        .product::<u16>();
    println!("{product}");

    Ok(())
}

fn solve(blueprint: [[u16; 3]; 4], minutes: u8) -> u16 {
    eprintln!("{blueprint:?}");
    let mut cache = HashSet::new();
    let mut best_value = 0;
    (0..2).for_each(|target| {
        solve_rec(
            &blueprint,
            minutes,
            target,
            [1, 0, 0, 0],
            [0, 0, 0, 0],
            &mut cache,
            &mut best_value,
        )
    });
    best_value
}

fn solve_rec(
    blueprint: &[[u16; 3]; 4],
    minutes: u8,
    target: u8,
    robots: [u8; 4],
    resources: [u16; 4],
    cache: &mut HashSet<(u8, u8, [u8; 4], [u16; 4])>,
    best_value: &mut u16,
) {
    if cache.contains(&(minutes, target, robots, resources)) {
        return;
    }
    cache.insert((minutes, target, robots, resources));
    let bound = resources.last().copied().unwrap()
        + robots.last().copied().unwrap() as u16 * minutes as u16
        + (minutes as u16 * (minutes.saturating_sub(1) as u16)); // very bad bound but well, the test passes
    if bound <= *best_value {
        return;
    }
    let for_target = &blueprint[target as usize];
    let minutes_waiting = for_target
        .iter()
        .zip(&resources)
        .map(|(needed, available)| needed.saturating_sub(*available))
        .zip(&robots)
        .filter(|&(needed, _)| needed > 0)
        .map(|(needed, per_turn)| {
            needed / *per_turn as u16 + if needed % *per_turn as u16 == 0 { 0 } else { 1 }
        })
        .max()
        .unwrap_or_default()
        + 1; // one extra for build time

    if minutes_waiting >= minutes as u16 {
        let value = resources.last().copied().unwrap()
            + robots.last().copied().unwrap() as u16 * minutes as u16;
        if value > *best_value {
            *best_value = value
        }
    } else {
        let mut new_resources = resources;
        new_resources
            .iter_mut()
            .zip(&robots)
            .for_each(|(res, rob)| *res += *rob as u16 * minutes_waiting);

        new_resources
            .iter_mut()
            .zip(&blueprint[target as usize])
            .for_each(|(res, req)| *res = res.checked_sub(*req).unwrap());
        let mut new_robots = robots;
        new_robots[target as usize] += 1;
        blueprint
            .iter()
            .enumerate()
            .rev()
            .filter(|&(_, print)| {
                print
                    .iter()
                    .zip(&robots)
                    .all(|(req, rob)| *req == 0 || *rob > 0)
            })
            .for_each(|(new_target, _)| {
                solve_rec(
                    blueprint,
                    minutes - minutes_waiting as u8,
                    new_target as u8,
                    new_robots,
                    new_resources,
                    cache,
                    best_value,
                )
            });
    }
}
