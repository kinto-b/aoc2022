//! # Monkey map
//!
//! We store the map as a Grid of chars. We stop when we
//! reach a '#', skip past ' ', and move one space for each
//! '.'.
//!
//! We need to find some way to label the six sides from 1 to 6
//! such that all pairs of neighbouring sides sum to less than 7.
//! Once we've done that, we'll know which sides are adjacent, so
//! we can create a mapping between side edge coordinates.

use std::{collections::HashMap, fs::read_to_string, ops::Index};

use crate::{grid::Grid, parse::parse_u32};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Right,
    Direction::Down,
    Direction::Left,
    Direction::Up,
];

type Position = (usize, usize);
type Map = Grid<char>;
type Instructions = (Vec<u32>, Vec<char>);

fn parse() -> (Map, Position, Instructions) {
    let input = read_to_string("data/day22.txt").unwrap();
    let mut lines = input.lines();

    // We'll need to pad each row to the width of the grid,
    let ncol = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| l.len())
        .max()
        .unwrap();

    let mut grid: Vec<char> = Vec::new();
    let mut line = lines.next().unwrap();
    let start_col = line.chars().position(|b| b == '.').unwrap();
    while !line.is_empty() {
        let row = format!("{:width$}", line, width = ncol);
        grid.append(&mut row.chars().collect::<Vec<char>>());
        line = lines.next().unwrap();
    }

    let nrow = grid.len() / ncol;
    let map = Grid::new(grid, nrow);

    // Parse instructions
    let instructions = lines.next().unwrap();
    let steps = parse_u32(instructions);
    let turns = instructions
        .chars()
        .filter(|x| ['L', 'R'].contains(x))
        .collect::<Vec<char>>();

    (map, (start_col, 0), (steps, turns))
}

/// Returns the position arrived at after taking a given number of steps
fn walk(start: Position, dir: &Direction, steps: usize, grid: &Grid<char>) -> Position {
    let (mut x, mut y) = start;
    let (mut xx, mut yy) = start;

    let mut s = 0;
    while s < steps {
        (xx, yy) = step((xx, yy), dir, (grid.nrow, grid.ncol));

        match grid.get(yy, xx) {
            '#' => break,
            ' ' => continue,
            '.' => {
                s += 1;
                (x, y) = (xx, yy);
            }
            _ => unreachable!(),
        }
    }

    (x, y)
}

/// Returns the location after taking one step, wrapping around the edges
fn step((x, y): Position, dir: &Direction, (nrow, ncol): Position) -> Position {
    match dir {
        Direction::Down => {
            if y < nrow - 1 {
                (x, y + 1)
            } else {
                (x, 0)
            }
        }
        Direction::Up => {
            if y > 0 {
                (x, y - 1)
            } else {
                (x, nrow - 1)
            }
        }
        Direction::Right => {
            if x < ncol - 1 {
                (x + 1, y)
            } else {
                (0, y)
            }
        }
        Direction::Left => {
            if x > 0 {
                (x - 1, y)
            } else {
                (ncol - 1, y)
            }
        }
    }
}

pub fn part1() -> usize {
    let (grid, mut start, (steps, turns)) = parse();
    let mut turns_todo = turns.iter();
    let mut facing = 0;

    for s in steps {
        start = walk(start, &DIRECTIONS[facing], s as usize, &grid);

        match turns_todo.next() {
            Some('R') => {
                if facing >= 3 {
                    facing = 0
                } else {
                    facing += 1
                }
            }
            Some('L') => {
                if facing >= 1 {
                    facing -= 1
                } else {
                    facing = 3
                }
            }
            _ => (),
        }
    }

    1000 * (start.1 + 1) + 4 * (start.0 + 1) + facing
}
}

pub fn part2() -> i32 {
    0
}
