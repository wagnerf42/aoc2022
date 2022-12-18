use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
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

fn inner_intervals(coordinates: &[i32]) -> impl Iterator<Item = std::ops::Range<i32>> + '_ {
    coordinates
        .iter()
        .tuple_windows()
        .map(|(c1, c2)| (c1 + 1)..*c2)
        .filter(|r| !r.is_empty())
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

    let mut xs: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    let mut ys: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    let mut zs: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| {
            l.split(',')
                .filter_map(|t| t.parse::<i32>().ok())
                .tuples()
                .next()
        })
        .for_each(|(x, y, z)| {
            xs.entry((y, z)).or_default().push(x);
            ys.entry((x, z)).or_default().push(y);
            zs.entry((x, y)).or_default().push(z);
        });

    xs.values_mut().for_each(|v| v.sort_unstable());
    ys.values_mut().for_each(|v| v.sort_unstable());
    zs.values_mut().for_each(|v| v.sort_unstable());

    let inner_points = xs
        .iter()
        .flat_map(|(&(y, z), vx)| inner_intervals(vx).flatten().map(move |x| (x, y, z)))
        .chain(
            ys.iter()
                .flat_map(|(&(x, z), vy)| inner_intervals(vy).flatten().map(move |y| (x, y, z))),
        )
        .chain(
            zs.iter()
                .flat_map(|(&(x, y), vz)| inner_intervals(vz).flatten().map(move |z| (x, y, z))),
        )
        .fold(HashMap::new(), |mut h: HashMap<(i32, i32, i32), u8>, p| {
            *h.entry(p).or_default() += 1;
            h
        })
        .into_iter()
        .filter_map(|(p, c)| if c == 3 { Some(p) } else { None })
        .collect::<HashSet<_>>();

    let inner_sides = points
        .iter()
        .flat_map(neighbours)
        .filter(|p| inner_points.contains(p))
        .count();
    println!("sides: {}", sides - inner_sides);

    Ok(())
}
