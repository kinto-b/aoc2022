//! Falling sand
//!
//! Examining the input, we see that all the x-coordinates fall in the range
//! 400-600 and all the y-coordinates fall in the range 0-200. This means
//! that we can store the positions of all rocks in a 200*200 length vector.
//!
//! This should be large enough to store all the sand locations we need for
//! part one. For part two, we need to be able to store, in the worst case
//! where there ~0 rocks in the cave, a pyramid of sand with height, H.
//! Such a pyramid will have a base width no greater than 2*H. So a
//! 500*200 length vector should be more than enough for both parts.

use std::fs::read_to_string;

use crate::grid::Grid;
use crate::parse;

const X_OFFSET: usize = 250; // Ensure sand source is at centre of grid
const GRID_WIDTH: usize = 500;
const GRID_HEIGHT: usize = 200;


/// Returns a tuple containing a grid representing the positions of the
/// rock and an integer representing the location of the bottom of the cave.
fn parse() -> (Grid<bool>, usize) {
    let input = read_to_string("data/day14.txt").unwrap();
    let points = input.lines().map(parse::parse_u32);

    let mut grid = Grid::new(vec![false; GRID_WIDTH * GRID_HEIGHT], GRID_HEIGHT);
    let mut bottom = 0;
    for line in points {
        bottom = bottom.max(*line.iter().skip(1).step_by(2).max().unwrap());

        for window in line.windows(4).step_by(2) {
            if let &[x0, y0, x1, y1] = window {
                for col in x1.min(x0)..=x1.max(x0) {
                    for row in y1.min(y0)..=y1.max(y0) {
                        grid.set(row as usize, col as usize - X_OFFSET, true);
                    }
                }
            }
        }
    }

    (grid, bottom as usize)
}

/// Returns the location a grain of sand comes to rest at after dripping from
/// the source
fn drip(grid: &Grid<bool>, bottom: usize) -> Option<(usize, usize)> {
    let (row0, col0) = (0, 500 - X_OFFSET); // Sand source
    let (mut row, mut col) = (row0, col0);
    let mut at_rest = false;

    while !at_rest {
        if row > bottom {
            return None; // Part one
        } else if !grid.get(row + 1, col) {
            (row, col) = (row + 1, col);
        } else if !grid.get(row + 1, col - 1) {
            (row, col) = (row + 1, col - 1);
        } else if !grid.get(row + 1, col + 1) {
            (row, col) = (row + 1, col + 1);
        } else if (row, col) == (row0, col0) {
            return None; // Part two
        } else {
            at_rest = true;
        }
    }

    Some((row, col))
}


/// Fills the grid by dripping sand from the source until either sand starts
/// falling away to infinity or sand is backed up to the source, and returns
/// number of grains of sand taken to fill the grid.
fn fill(grid: &mut Grid<bool>, bottom: usize) -> u32 {
    let mut count = 0;
    while let Some((row, col)) = drip(grid, bottom) {
        grid.set(row, col, true);
        count += 1;
    }
    count
}

/// Returns the number of grains of sand taken to fill the cave
pub fn part1() -> u32 {
    let (mut grid, bottom) = parse();
    fill(&mut grid, bottom)
}

/// Returns the number of grains of sand taken to fill the cave with a floor
pub fn part2() -> u32 {
    let (mut grid, mut bottom) = parse();

    // Add cave floor
    bottom += 2;
    for i in 0..grid.ncol {
        grid.set(bottom, i, true);
    }

    fill(&mut grid, bottom) + 1
}

