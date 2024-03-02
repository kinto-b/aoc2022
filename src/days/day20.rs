//! # Shifting list elements
//!
//! We can solve part one fairly quickly using only arrays by rotating subslices.
//! We keep track of which elements we've already shifted using a second array
//! of booleans that we shift to mirror the original array. For example
//!
//!  *start*   ->  *flag*   -> *shift*
//!  [1, 2, 3] -> ......... -> [2, 1, 3]
//!  [F, F, F] -> [T, F, F] -> [F, T, F]
//!
//! Moving left to right across the list, skipping visited nodes, preserves the
//! original list order, so we only need linear time to iterate through the
//! elements.
//!
//! However, an array based solution is very slow for part two. The main
//! bottle-neck is that we need to do an O(n) search for each element in the
//! list on each pass. So:
//!
//! TODO: Try using a tree for part two.

use std::fs::read_to_string;

const GROVE_DELTAS: [usize; 3] = [1000, 2000, 3000];
const DECRYPTION_KEY: i64 = 811589153;

fn parse() -> Vec<i64> {
    let input = read_to_string("data/day20.txt").unwrap();
    input.lines().map(|s| s.parse::<i64>().unwrap()).collect()
}

pub fn part1() -> i64 {
    let mut list = parse();
    mix_once(&mut list);
    grove_coordinates(&list)
}

pub fn part2() -> i64 {
    let mut list: Vec<(usize, i64)> = parse()
        .iter()
        .map(|i| *i * DECRYPTION_KEY)
        .enumerate()
        .collect();

    mix(&mut list, 10);

    let list_bare: Vec<i64> = list.iter().map(|(_, x)| x).cloned().collect();
    grove_coordinates(&list_bare)
}

fn grove_coordinates(list: &[i64]) -> i64 {
    let origin = list.iter().position(|x| *x == 0).unwrap();
    GROVE_DELTAS
        .map(|d| list[(origin + d) % list.len()])
        .iter()
        .sum()
}

/// Mix the provided list a given number of times
fn mix(list: &mut Vec<(usize, i64)>, times: usize) {
    let n = list.len();
    for _ in 0..times {
        for p in 0..n {
            let fr = list
                .iter()
                .position(|(priority, _)| *priority == p)
                .unwrap();

            let to = match wrap(fr as i64 + list[fr].1, n - 1) {
                0 => n - 1,
                x if x == n - 1 => 0,
                x => x,
            };

            if fr < to {
                list[fr..=to].rotate_left(1);
            } else {
                list[to..=fr].rotate_right(1);
            }
        }
    }
}

/// Mixes the provided list once
fn mix_once(list: &mut Vec<i64>) {
    let mut visited = vec![false; list.len()];

    let n = list.len();
    let mut i = 0;
    while i < n {
        if visited[i] {
            i += 1;
            continue;
        }
        visited[i] = true;

        // Endpoints wrap 'early'
        let j = match wrap(i as i64 + list[i], n - 1) {
            0 => n - 1,
            x if x == n - 1 => 0,
            x => x,
        };

        if i < j {
            list[i..=j].rotate_left(1);
            visited[i..=j].rotate_left(1);
        } else {
            list[j..=i].rotate_right(1);
            visited[j..=i].rotate_right(1);
        }
    }
}

/// Return the index within 0..n after wrapping around either side
fn wrap(i: i64, n: usize) -> usize {
    if i < 0 {
        n - ((i.unsigned_abs() as usize) % n)
    } else {
        (i.unsigned_abs() as usize) % n
    }
}
