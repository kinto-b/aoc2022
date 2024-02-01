
// TODO: Would be nice not to have to do the iter->vec->iter round trip.
fn parse() -> Vec<[u8; 4]>{
    include_str!("../../data/day04.txt")
        .lines()
        .map(parse_line)
        .collect()
}

fn parse_line(x: &str) -> [u8; 4] {
    x.split_terminator(['-', ','])
        .map(|i| i.parse::<u8>().unwrap())
        .collect::<Vec<u8>>()[..4]
        .try_into()
        .unwrap()
}

/// Returns true if either pair is fully contained by the other, else false
fn full_overlap([l1, u1, l2, u2]: &[u8; 4]) -> bool {
    ((l1 <= l2) && (u1 >= u2)) || // right in left
    ((l2 <= l1) && (u2 >= u1))    // left in right
}

/// Returns true if either pair overlaps with the other, else false
fn partial_overlap([l1, u1, l2, u2]: &[u8; 4]) -> bool {
    ((l1 <= l2) & (u1 >= l2)) || // ( [ ) ]
    ((l2 <= l1) & (u2 >= l1))    // [ ( ] )
}

/// Count the number of 'contained' pairs
pub fn part1() -> usize {
    parse().iter()
        .filter(|&v| full_overlap(v))
        .count()
}

pub fn part2() -> usize {
    parse().iter()
        .filter(|&v| partial_overlap(v))
        .count()
}