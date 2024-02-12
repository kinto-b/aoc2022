//! Hill climbing (in reverse!) with BFS
//!
//! Since the grid defines an undirected graph, the distance from 'S' to 'E' is
//! the same as the distance from 'E' to 'S'. Likewise, the shortest distance
//! from an 'a' to 'E' is the same as the shortest distance from 'E' to an 'a'.
//!
//! Both problems can be solved using a breadth first search.

use crate::grid::Grid;
use std::fs::read_to_string;

type Point = (usize, usize);

fn parse() -> (Grid<u8>, Point) {
    let input = read_to_string("data/day12.txt").unwrap();
    let grid = Grid::parse(&input);
    let start = grid.find(b'E'); // Start at the end

    (grid, start.unwrap())
}

/// Returns the minimal distance from start to end through the grid
fn bfs(grid: &mut Grid<u8>, start: Point, end: u8) -> u32 {
    let mut queue: Vec<(usize, usize)> = vec![start];
    let mut next: Vec<(usize, usize)> = Vec::new();

    let mut visited = Grid::new(vec![false; grid.len()], grid.nrow);
    visited.set(start.0, start.1, true);

    let mut nsteps = 0;
    loop {
        while let Some((i, j)) = queue.pop() {
            let curr = grid.get(i, j);
            if curr == end {
                return nsteps;
            }

            for (ni, nj) in grid.neighbours(i, j) {
                if !visited.get(ni, nj) & connected(curr, grid.get(ni, nj)) {
                    visited.set(ni, nj, true);
                    next.push((ni, nj));
                }
            }
        }

        nsteps += 1;
        queue.append(&mut next);
    }
}

/// Returns true if `x` is no more than one step higher than `y`
fn connected(x: u8, y: u8) -> bool {
    height(x) as i32 - height(y) as i32 <= 1
}

/// Returns the height associated with a given element
fn height(x: u8) -> u8 {
    match x {
        b'S' => b'a',
        b'E' => b'z',
        b => b,
    }
}

/// Return the minimal number of steps from 'E' to 'S'
pub fn part1() -> u32 {
    let (mut grid, start) = parse();
    bfs(&mut grid, start, b'S')
}

/// Return the minimal number of steps from 'E' to an 'a'
pub fn part2() -> u32 {
    let (mut grid, start) = parse();
    bfs(&mut grid, start, b'a')
}
