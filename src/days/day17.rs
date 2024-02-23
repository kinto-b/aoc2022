//! Tetris
//! 
//! We can store each layer as an 8-bit integer where on-bits represent 
//! occupied space. Each rock may then be represented by at most 4 such 
//! integers. We move the rock left and right by bit shifting. We move the 
//! rock down simply by decrementing its position. Every time we move the rock
//! we check that the space it moves into isn't already occupied by comparing
//! the rock bits with the bits of the context it's moving into.
//!
//! In theory the second part is challenging because you've got to keep track
//! of the state of the chamber. However, in practice, it turns out that the
//! state of the chamber doesn't matter! All we care about is finding out
//! when the rock/jet cycle begins to loop. So we initialise an RxJ grid, where
//! R is the length of the rock cycle and J is the length of the jet cycle. We 
//! use the grid to keep track of the step on which each rock/jet combo was
//! observed and the height of the tower at that time. If we observe a combo
//! re-appear, then we've hit a cycle, so we can fast-forward.


use std::iter::zip;
// use std::collections::hash_map::Entry;
use crate::grid::Grid;

// Constants -------------------------------------------------------------------

/// Rocks have a certain shape (defined by the positions they take up) and size
/// (the number of rows they span).
struct Rock {
    occupying: [u8; 4],
    height: usize
}

const FLOOR: u8 = 0b11111111;
const EMPTY: u8 = 0b00000001; // Chamber is only 7 spaces wide

const ROCKS: [Rock; 5] = [
    // Flat
    Rock { occupying: [
        0b00111100, // Bottom
        0b00000000,
        0b00000000,
        0b00000000, // Top
    ], height: 1},

    // Plus
    Rock { occupying: [
        0b00010000,
        0b00111000,
        0b00010000,
        0b00000000,
    ], height: 3},
    
    // Backwards L
    Rock { occupying: [
        0b00111000,
        0b00001000,
        0b00001000,
        0b00000000,
    ], height: 3},

    // Stick
    Rock { occupying: [
        0b00100000,
        0b00100000,
        0b00100000,
        0b00100000,
    ], height: 4},

    // Box
    Rock { occupying: [
        0b00110000,
        0b00110000,
        0b00000000,
        0b00000000,
    ], height: 2}
];

// Solution --------------------------------------------------------------------

#[derive(Debug)]
enum Direction {
    Left,
    Right
}

/// Returns the input as a vector of Directions
fn parse() -> Vec<Direction> {
    include_bytes!("../../data/day17.txt").iter().
        map(|b| {
            match b {
                b'<' => Direction::Left,
                b'>' => Direction::Right,
                _ => unreachable!()
            }
        })
        .collect()
}


impl Rock {
    /// Pushes the rock in a Direction if it's not blocked
    fn push(&mut self, dir: &Direction, context: &[u8]) {
        let mut occupying = self.occupying;

        // Try first
        match dir {
            Direction::Left => {
                for i in occupying.iter_mut() {
                    *i <<= 1;
                }
            },

            Direction::Right => {
                for i in occupying.iter_mut() {
                    *i >>= 1;
                }
            }
        }

        // Check that we haven't gone over the edge
        if zip(occupying, self.occupying).any(|(a, b)| a.count_ones() != b.count_ones()) {
            return;
        }

        // Check that we're not occupying already occupied space
        if overlap(&occupying, context) {
            return;
        }

        // Otherwise we're good :)
        self.occupying = occupying;
    }
}

/// Returns true if the bits of x and y overlap, elementwise
fn overlap(x: &[u8], y: &[u8]) -> bool {
    zip(x, y).any(|(a, b)| a & b > 0)
}

/// Simulates the rocks falling, subject to the jets of air, across a given 
/// number of iterations
fn simulate(input: Vec<Direction>, iterations: usize) -> usize {
    let mut jets = input.iter().enumerate().cycle();
    let mut rocks = ROCKS.iter().enumerate().cycle();
    
    let mut cache_step = Grid::new(vec![0; input.len() * ROCKS.len()], ROCKS.len());
    let mut cache_height = Grid::new(vec![0; input.len() * ROCKS.len()], ROCKS.len());

    let mut chamber: Vec<u8> = vec![FLOOR];
    let mut tower_height = 0;

    for i in 1..(iterations+1) {
        // Add enough space to fit new rock
        if tower_height+8 > chamber.len() {
            chamber.resize(tower_height+8, EMPTY)
        }
        
        // Spawn rock
        let (r, Rock {occupying, height})= rocks.next().unwrap();
        let mut rock = Rock { occupying: *occupying, height: *height};
        let mut rock_pos = tower_height+4;
        
        // Drop rock
        let mut context = &chamber[rock_pos..(rock_pos+4)];
        loop {
            let (j, jet) = jets.next().unwrap();

            // Check cache and fastforward
            let i0 = cache_step.get(r, j);
            if i0 > 0 {
                let h0 = cache_height.get(r, j);
                let remaining = iterations - i;
                let cycle_length = i-i0;
                let (d, m) = (remaining / cycle_length, remaining % cycle_length);
                if m == 0 {
                    return tower_height + (tower_height - h0)*d;
                }
            } else {
                cache_step.set(r, j, i);
                cache_height.set(r, j, tower_height);
            }

            // Move rock
            rock.push(jet, context);
            context = &chamber[(rock_pos-1)..(rock_pos+3)];

            if overlap(&rock.occupying, context) {
                // Add to tower at current location
                for i in 0..rock.height {
                    chamber[rock_pos + i] |= rock.occupying[i];
                }

                tower_height = tower_height.max(rock_pos + rock.height - 1);
                break;
            }
            rock_pos -= 1
        }        
    }

    tower_height
}

pub fn part1() -> usize {
    let input = parse();
    simulate(input, 2022)
}

pub fn part2() -> usize {
    let input = parse();
    simulate(input, 1_000_000_000_000)
}
