use std::{
    fs::File,
    io::{BufRead, BufReader},
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

trait Graph {
    type Vertex;
    type Neighbours<'b>;
    fn neighbours<'b>(&self, vertex: &Self::Vertex) -> Self::Neighbours<'b>;
}

impl Graph for Vec<Vec<u8>> {
    type Vertex = Pos;
    fn neighbours<'b>(&self, vertex: &Self::Vertex) -> Self::Neighbours<'b> {
        todo!()
    }
}

fn main() -> std::io::Result<()> {
    let map = load_map("input")?;
    Ok(())
}
