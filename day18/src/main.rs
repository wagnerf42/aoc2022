use itertools::Itertools;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn neighbours(point: &(i32, i32, i32)) -> impl Iterator<Item = (i32, i32, i32)> {
    let (x, y, z) = *point;
    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
    .into_iter()
}

fn main() -> std::io::Result<()> {
    let points: HashSet<(i32, i32, i32)> = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| {
            l.split(',')
                .filter_map(|t| t.parse::<i32>().ok())
                .tuples()
                .next()
        })
        .collect();
    let sides = points
        .iter()
        .flat_map(neighbours)
        .filter(|p| !points.contains(p))
        .count();
    println!("sides: {sides}");

    let xlimits = points
        .iter()
        .copied()
        .map(|(x, _, _)| x)
        .minmax()
        .into_option()
        .unwrap_or_default();

    let ylimits = points
        .iter()
        .copied()
        .map(|(_, y, _)| y)
        .minmax()
        .into_option()
        .unwrap_or_default();

    let zlimits = points
        .iter()
        .copied()
        .map(|(_, _, z)| z)
        .minmax()
        .into_option()
        .unwrap_or_default();

    let external_points = fill(&points, xlimits, ylimits, zlimits);

    let external_sides = points
        .iter()
        .flat_map(neighbours)
        .filter(|p| external_points.contains(p))
        .count();
    println!("external sides: {external_sides}");

    Ok(())
}

fn fill(
    points: &HashSet<(i32, i32, i32)>,
    xlimits: (i32, i32),
    ylimits: (i32, i32),
    zlimits: (i32, i32),
) -> HashSet<(i32, i32, i32)> {
    let xrange = (xlimits.0 - 1)..(xlimits.1 + 2);
    let yrange = (ylimits.0 - 1)..(ylimits.1 + 2);
    let zrange = (zlimits.0 - 1)..(zlimits.1 + 2);
    let start = (xrange.start, yrange.start, zrange.start);
    let mut stack = vec![start];
    let mut seen = HashSet::new();
    seen.insert(start);
    std::iter::from_fn(|| {
        if let Some(current_point) = stack.pop() {
            stack.extend(neighbours(&current_point).filter(|n| {
                let r = !points.contains(n)
                    && !seen.contains(n)
                    && xrange.contains(&n.0)
                    && yrange.contains(&n.1)
                    && zrange.contains(&n.2);
                if r {
                    seen.insert(*n);
                }
                r
            }));
            Some(current_point)
        } else {
            None
        }
    })
    .collect()
}
