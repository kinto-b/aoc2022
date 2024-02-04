//! Snaking ropes

use std::{collections::HashSet, fs::read_to_string};

/// Convert a step character into a vector (dx, dy)
fn step(x: &str) -> (i32, i32) {
    match x {
        "U" => { (0, 1) },
        "D" => { (0, -1) },
        "L" => { (-1, 0) },
        "R" => { (1, 0) },
        _ => unreachable!()
    }
}

/// Returns a new tail coord after catching up with the head
fn catchup((xh, yh): (i32, i32), (xt, yt): (i32, i32)) -> (i32, i32) {
    let dx = xh - xt;
    let dy = yh - yt;

    if dx.abs() > 1 || dy.abs() > 1 {
        (xt + dx.signum(), yt + dy.signum())
    } else {
        (xt, yt)
    }
}

/// Return the number of locations visited by the tail of a rope with a given 
/// number of knots whose head follows a given set of directions.
fn snake(knots: usize) -> usize {
    let directions: String = read_to_string("data/day09.txt").unwrap();

    let (mut x, mut y) = (0, 0);  // Head position
    let mut rope = vec![(0, 0); knots-1]; // Remaining knots
    
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((x, y));

    for l in directions.lines() {
        if let Some((d, s)) = l.split_once(' ') {
            let (dx, dy) = step(d);
            let nsteps = s.parse::<usize>().unwrap();
            for _ in 0..nsteps {
                x += dx;
                y += dy;

                rope = rope.iter().scan((x, y), |state, &(u, v)| {
                    *state = catchup(*state, (u, v));
                    Some(*state)
                }).collect();
                
                visited.insert(*rope.last().unwrap());
            }            
        }
    }

    visited.len()
}

/// Returns the number of places visited by T in a two-knot rope
pub fn part1() -> usize {
    snake(2)
}


/// Returns the number of places visited by T in a ten-knot rope
pub fn part2() -> usize {
    snake(10)   
}