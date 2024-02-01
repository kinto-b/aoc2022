use std::collections::{hash_map::RandomState, HashSet};

/// Returns a tuple of sets representing the compartments
fn compartments(x: &[u8]) -> (HashSet<u8>, HashSet<u8>) {
    let (left, right) = x.split_at(x.len() / 2);
    (
        HashSet::from_iter(left.iter().cloned()), 
        HashSet::from_iter(right.iter().cloned())
    )
}

/// Returns the priority corresponding to a byte character
fn priority(x: u8) -> i32 {
    if x <= b'Z' { (x - b'A' + 27) as i32 } else { (x - b'a' + 1) as i32 }
}

/// Returns bag priority based on contents of left and right compartments
fn bag_priority(left: HashSet<u8>, right: HashSet<u8>) -> i32 {
    priority(*left.intersection(&right).next().unwrap())
}

/// Returns badge priority based on contents of the three backpacks in the group
fn badge_priority(x: &[u8], y: &[u8], z: &[u8]) -> i32 {
    let x: HashSet<u8, RandomState> = HashSet::from_iter(x.iter().cloned());
    let y: HashSet<u8, RandomState> = HashSet::from_iter(y.iter().cloned());

    let badge = z.iter()
        // .filter(|&i| (i >= &b'A') & (i <= &b'z'))
        .filter(|&i| x.contains(i))
        .filter(|&i| y.contains(i))
        .next()
        .unwrap();

    priority(*badge)
}

/// Sum of bag priorities
pub fn part1() -> i32 {
    include_bytes!("../../data/day03.txt")
        .split(|b| *b == b'\n')
        .map(compartments)
        .map(|(x, y)| bag_priority(x, y))
        .sum()
}

/// Sum of badge priorities
pub fn part2() -> i32 {
    let mut elves = include_bytes!("../../data/day03.txt")
        .split(|b| *b == b'\n');

    let mut solution = 0;

    while let Some(e1) = elves.next() {
        solution += badge_priority(e1, elves.next().unwrap(), elves.next().unwrap())
    }

    solution
}