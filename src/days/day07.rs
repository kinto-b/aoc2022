use std::{collections::BTreeMap, fs::read_to_string};

/// Returns the sizes of each directory in the tree
fn parse() -> BTreeMap<String, u32> {
    let input = read_to_string("data/day07.txt").unwrap();

    let mut directory_sizes: BTreeMap<String, u32> = BTreeMap::new();
    let mut path: Vec<String> = Vec::new();

    for line in input.lines() {
        let x = line.replace("$ ", "");
        let mut couplet = x.split_whitespace();
        let prefix = couplet.next().unwrap();

        match prefix {
            "cd" => change_directory(&mut path, couplet.next().unwrap()),
            "ls" => (),
            x if x.starts_with('d') => (),

            // If prefix isn't matched yet, then it's a filesize!
            _ => add_filesize(&mut directory_sizes, &path, prefix),
        }
    }

    directory_sizes
}

/// Updates path according to the command
fn change_directory(path: &mut Vec<String>, commands: &str) {
    for cmd in commands.lines().map(|s| s.replace("$ cd ", "")) {
        match cmd.as_str() {
            ".." => {
                path.pop();
            }
            "/" => {
                path.clear();
                path.push(String::from("."));
            }
            _ => {
                path.push(cmd);
            }
        }
    }
}

/// Updates sizes by adding the file size to each directory in the path
fn add_filesize(sizes: &mut BTreeMap<String, u32>, path: &Vec<String>, file: &str) {
    let size = file.parse::<u32>().unwrap();
    let mut d = String::from("");

    for p in path {
        d.push_str(p);
        d.push('/');

        sizes
            .entry(d.clone())
            .and_modify(|s| *s += size)
            .or_insert(size);
    }
}

/// Returns the sum of the sizes of all directories with sizes < 100,000
pub fn part1() -> u32 {
    let sizes = parse();

    sizes.values().filter(|&s| *s < 100_000).sum()
}

/// Returns the size of the smallest directory we need to delete to
/// reduce occupied disk space below 40,000,000
pub fn part2() -> u32 {
    let sizes = parse();
    let delete_size = sizes.get("./").unwrap() - 40_000_000;

    *sizes.values().filter(|&s| *s > delete_size).min().unwrap()
}
