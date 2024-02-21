//! Optimisation
//!
//! 

use crate::grid::Grid;
use crate::parse;
use std::{cmp::Reverse, collections::HashMap, fs::read_to_string};

// Parsing --------------------------------------------------------------------
struct Cave {
    valves: Vec<Valve>,
    proximity: Grid<u32>
}

struct Valve {
    name: String,
    rate: u32,
    tunnels: Vec<String>,
}

impl Valve {
    fn parse(line: &str) -> Self {
        let node_chars = line
            .chars()
            .skip(1)
            .filter(|x| x.is_ascii_uppercase())
            .collect::<Vec<char>>();

        let mut nodes = node_chars.chunks(2).map(|x| x.iter().collect::<String>());

        let name = nodes.next().unwrap();
        let tunnels: Vec<String> = nodes.collect();

        let rate: u32 = parse::parse_u32(line)[0];

        Valve {
            name,
            rate,
            tunnels,
        }
    }
}

/// Returns a vector of Valves and a Grid containing their proximities to one another
fn parse() -> Cave {
    let input = read_to_string("data/day16.txt").unwrap();

    // Read all valves from input
    let mut valves: Vec<Valve> = input.lines().map(Valve::parse).collect();

    // Create a full graph
    let mut graph = HashMap::new();
    for v in &valves {
        graph.insert(v.name.clone(), v.tunnels.clone());
    }

    // Now strip out the zero rate valves
    valves.retain(|v| v.rate > 0 || v.name == "AA");
    
    // Sort valves by rate, descending order
    valves.sort_by_key(|v| Reverse(v.rate));

    // ... and use the graph to find min distances between remaining valves
    let mut proximity = Grid::new(vec![u32::MAX; valves.len() * valves.len()], valves.len());
    for (i, a) in valves.iter().enumerate() {
        for (j, b) in valves.iter().enumerate() {
            if proximity.get(i, j) < u32::MAX {
                continue;
            }

            // Proximity matrix should be symmetric
            let dist = bfs(&graph, &a.name, &b.name);
            proximity.set(i, j, dist);
            proximity.set(j, i, dist);
        }
    }

    Cave { valves, proximity }
}

/// Return shortest path through the graph from start to end, computed via BFS
fn bfs(graph: &HashMap<String, Vec<String>>, start: &str, end: &str) -> u32 {
    let mut queue = vec![start];
    let mut visited = Vec::new();
    let mut next: Vec<&str> = Vec::new();

    let mut dist = 0;
    loop {
        while let Some(node) = queue.pop() {
            if node == end {
                return dist;
            } else if visited.contains(&node) {
                continue;
            }

            visited.push(node);
            graph.get(node).unwrap().iter().for_each(|v| next.push(v));
        }

        queue.append(&mut next);
        dist += 1;
    }
}

// Solutions ------------------------------------------------------------------

/// Returns maximal pressure release, computed via branch and bound search
pub fn part1() -> u32 {
    let cave = parse();
    let loc = cave.valves.len() - 1;

    // Use a bit set to represent the open valves
    let still_open = (0..loc).fold(0, |set, i| set | 1 << i);

    // Define the initial state to pass to the branch and bound algorithm
    let state = State {
        loc,
        still_open,
        time: 30,
        score: 0,
    };

    // We'll prune the graph by getting rid of branches whose upper bound
    // scores are lower than the cutoff. This needs to be a closure like this
    // so that we can reuse `branch_and_bound` for part two
    let mut best = 0;
    let mut cutoff = |_, score: u32| {
        best = best.max(score);
        best
    };
    branch_and_bound(&state, &cave, &mut cutoff);

    best
}

pub fn part2() -> u32 {
    let cave = parse();
    let loc = cave.valves.len() - 1;

    // Use a bit set to represent the open valves
    let still_open = (0..loc).fold(0, |set, i| set | 1 << i);

    // Define the initial state to pass to the branch and bound algorithm
    let state = State {
        loc,
        still_open,
        time: 26,
        score: 0,
    };

    // We want to explore the entire search tree without pruning anything, and
    // store all the results. This gives us high-scores for all possible
    // subsets
    let mut cache = vec![0; still_open + 1];
    let mut cutoff = |still_open: usize, score: u32| {
        cache[still_open] = cache[still_open].max(score);
        0
    };
    branch_and_bound(&state, &cave, &mut cutoff);

    // We can now iterate over all the disjoint subsets, and take the max score
    let mut best = 0;
    let scores: Vec<_> = cache.iter().enumerate().filter(|(_, &s)| s > 0).collect();
    for i in 0..scores.len() {
        let (m1, &s1) = scores[i];
        for (m2, &s2) in scores.iter() {
            // m1 and m2 denote the valves _not_ in the set. We get the valves
            // in each set by xor'ing against the full set of valves.
            if (still_open ^ m1) & (still_open ^ m2) == 0 {
                best = best.max(s1+s2);
            }
        }
    }

    best
}

// Helpers --------------------------------------------------------------------
#[derive(Debug)]
struct State {
    loc: usize,
    still_open: usize,
    time: u32,
    score: u32,
}

/// Return the next state after moving to the valve with index k
fn open(loc: usize, state: &State, cave: &Cave) -> Option<State> {
    let d = cave.proximity.get(state.loc, loc) + 1;
    if d > state.time {
        return None;
    }

    let time = state.time - d;
    Some(State {
        loc,
        still_open: state.still_open ^ (1 << loc),
        time,
        score: state.score + cave.valves[loc].rate * time,
    })
}

/// Return score after opening each remaining valve in decreasing rate order, 
/// taking 2 mins per valve
fn upper_bound(state: &State, cave: &Cave) -> u32 {
    let State {still_open, loc: _, mut time, mut score} = state;

    let mut next_open = *still_open;
    while (next_open > 0) & (time >= 2 ){
        let i = next_open.trailing_zeros() as usize;
        next_open ^= 1 << i;

        time -= 2;
        score += cave.valves[i].rate * time;
    }

    score
}

/// Explore the branches recursively, A->AB,AC,AD,...,->ABC,ABD,...->...
fn branch_and_bound(state: &State, cave: &Cave, cutoff: &mut impl FnMut(usize, u32)->u32) {
    let mut still_open = state.still_open;    
    let best = cutoff(still_open, state.score);

    while still_open > 0 {
        let pos = still_open.trailing_zeros() as usize;
        still_open ^= 1 << pos;

        if let Some(s) = open(pos, state, cave) {
            if upper_bound( &s, cave) < best {
                continue; // Prune branch
            }

            branch_and_bound(&s, cave, cutoff);
        }
    }
}
