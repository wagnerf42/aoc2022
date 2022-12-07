use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
};

#[derive(Debug)]
enum Entry {
    File(u32, PathBuf),
    Dir(PathBuf),
}

enum InputLine {
    Cd(String),
    Entry(Entry),
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let start = tokens.next().ok_or(())?;
        if start == "dir" {
            tokens
                .next()
                .ok_or(())
                .map(|dirname| Entry::Dir(dirname.into()))
        } else {
            start.parse::<u32>().map_err(|_| ()).and_then(|size| {
                tokens
                    .next()
                    .ok_or(())
                    .map(|filename| Entry::File(size, filename.into()))
            })
        }
    }
}

impl FromStr for InputLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("$ cd") {
            Ok(InputLine::Cd(s[5..].to_string()))
        } else if s.starts_with("$ ls") {
            Err(()) // useless, we just discard it through err
        } else {
            s.parse::<Entry>().map(InputLine::Entry)
        }
    }
}

fn fill_sizes(
    sizes: &mut HashMap<PathBuf, u32>,
    dirs: &HashMap<PathBuf, Vec<Entry>>,
    current_path: &PathBuf,
) -> u32 {
    sizes.get(current_path).cloned().unwrap_or_else(|| {
        let computed_size = dirs
            .get(current_path)
            .map(|entries| {
                entries
                    .iter()
                    .map(|e| match e {
                        Entry::File(size, _) => *size,
                        Entry::Dir(dir_name) => {
                            let mut absolute_dir = current_path.to_owned();
                            absolute_dir.push(dir_name);
                            fill_sizes(sizes, dirs, &absolute_dir)
                        }
                    })
                    .sum::<u32>()
            })
            .unwrap();
        sizes.insert(current_path.to_owned(), computed_size);
        computed_size
    })
}

fn main() -> std::io::Result<()> {
    let (_, dirs): (_, HashMap<PathBuf, Vec<Entry>>) = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<InputLine>().ok())
        .fold((PathBuf::new(), HashMap::new()), |(mut cwd, mut h), l| {
            match l {
                InputLine::Cd(path) => match path.as_str() {
                    "/" => cwd.clear(),
                    ".." => {
                        cwd.pop();
                    }
                    s => {
                        cwd.push(s);
                    }
                },
                InputLine::Entry(e) => h.entry(cwd.clone()).or_default().push(e),
            }
            (cwd, h)
        });
    let mut sizes = HashMap::new();
    let root = PathBuf::new();
    fill_sizes(&mut sizes, &dirs, &root);
    let s: u32 = sizes.values().filter(|&&s| s <= 100_000).sum();
    println!("sum of dirs < 100_000 : {}", s);

    // note that we could be more optimal by walking the tree
    // but we still need to walk the whole tree to compute the sizes in the first place
    let free_space = 70_000_000 - sizes.get(&root).unwrap();
    let s: u32 = *sizes
        .values()
        .filter(|&&s| free_space + s >= 30_000_000)
        .min()
        .unwrap();
    println!("min dir large enough: {}", s);
    Ok(())
}
