use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Chain,
    option::IntoIter,
    path::Path,
};

fn load_map<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<Vec<u8>>> {
    Ok(BufReader::new(File::open(path)?)
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|c| match c {
                    b'a'..=b'z' => *c,
                    b'E' => b'z' + 1,
                    b'S' => b'a' - 1,
                    _ => panic!("invalid char in input"),
                })
                .collect()
        })
        .collect())
}

struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn neighbours(
        &self,
        xmax: usize,
        ymax: usize,
    ) -> Chain<Chain<Chain<IntoIter<Pos>, IntoIter<Pos>>, IntoIter<Pos>>, IntoIter<Pos>> {
        self.x
            .checked_sub(1)
            .map(|x| Pos { x, y: self.y })
            .into_iter()
            .chain(self.y.checked_sub(1).map(|y| Pos { x: self.x, y }))
            .chain((self.x + 1 < xmax).then_some(Pos {
                x: self.x + 1,
                y: self.y,
            }))
            .chain((self.y + 1 < ymax).then_some(Pos {
                x: self.x,
                y: self.y + 1,
            }))
    }
}

fn height_diff(grid: &[Vec<u8>], pos1: &Pos, pos2: &Pos) -> u8 {
    todo!()
}

struct NFilter<'g, G: Graph, I> {
    graph: &'g G,
    iter: I,
    n1: &'g G::Vertex,
}

impl<'g, G: Graph, I: Iterator<Item = G::Vertex>> Iterator for NFilter<'g, G, I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .filter(|n2| self.graph.are_neighbours(self.n1, n2))
    }
}

trait Graph {
    type Vertex;
    type Neighbours<'b>
    where
        Self: 'b;
    fn are_neighbours(&self, v1: &Self::Vertex, v2: &Self::Vertex) -> bool;
    fn neighbours<'b>(&'b self, vertex: &'b Self::Vertex) -> Self::Neighbours<'b>;
}

impl Graph for Vec<Vec<u8>> {
    type Vertex = Pos;
    type Neighbours<'b> = NFilter<
        'b,
        Self,
        Chain<Chain<Chain<IntoIter<Pos>, IntoIter<Pos>>, IntoIter<Pos>>, IntoIter<Pos>>,
    >;
    fn neighbours<'b>(&'b self, vertex: &'b Self::Vertex) -> Self::Neighbours<'b> {
        let ymax = self.len();
        let xmax = self[0].len();
        NFilter {
            graph: self,
            iter: vertex.neighbours(xmax, ymax),
            n1: vertex,
        }
    }

    fn are_neighbours(&self, v1: &Self::Vertex, v2: &Self::Vertex) -> bool {
        height_diff(self, v1, v2) <= 1
    }
}

fn main() -> std::io::Result<()> {
    let map = load_map("input")?;
    Ok(())
}
