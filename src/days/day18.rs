//! Counting cubes
//!
//! Cubes are adjacent if two of three coordinates are the same and a third
//! differs by exactly one. If we sort the cubes by their coordinates and then
//! iterate along them, we can count if each cube neighbours the following one
//! by checking if the first two coordinates match and the third differs by
//! exactly one. Since 'neighbouring' is symmetric, the total number of
//! neighbours along the third dimension will be equal to twice this count.
//!
//! We can do the same for each dimension and then compute the number of exposed
//! faces as 6C-2a-2b-2c where C is the number of cubes and a, b and c are the
//! counts along the first second and third dimensions.
//!
//! For part two, we'll use a flooding algorithm. Starting from some arbitrary
//! position outside of the lava droplet, we 'colour' each adjacent cube which
//! is not a lava droplet.

use std::fs::read_to_string;

#[derive(Debug)]
struct Cube {
    position: (usize, usize, usize),
}

impl Cube {
    /// Parse text like "12,1,3" and return a Cube
    fn parse(l: &str) -> Self {
        let coord: Vec<usize> = l
            .split_terminator(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        Cube {
            position: (coord[0], coord[1], coord[2]),
        }
    }

    /// Rotate position so that (x, y, z) -> (z, x, y)
    fn rotate_right(&mut self) {
        let pos = self.position;
        self.position = (pos.2, pos.0, pos.1)
    }
}

fn parse() -> Vec<Cube> {
    let input = read_to_string("data/day18.txt").unwrap();
    input.lines().map(Cube::parse).collect()
}

pub fn part1() -> usize {
    let mut cubes = parse();
    let cube0 = Cube {
        position: (99, 99, 99),
    };

    let mut neighbours = 0;

    cubes.sort_by_key(|cb| cb.position);
    neighbours += count_neighbours(&cubes[..], &cube0);

    for cb in cubes.iter_mut() {
        cb.rotate_right();
    }
    cubes.sort_by_key(|cb| cb.position);
    neighbours += count_neighbours(&cubes[..], &cube0);

    for cb in cubes.iter_mut() {
        cb.rotate_right();
    }
    cubes.sort_by_key(|cb| cb.position);
    neighbours += count_neighbours(&cubes[..], &cube0);

    (cubes.len() * 6) - 2 * neighbours
}

fn count_neighbours(cubes: &[Cube], cube0: &Cube) -> usize {
    cubes
        .iter()
        .scan(cube0, |state, cube| {
            let neighbouring = _neighbours(state, cube);
            *state = cube;

            if neighbouring {
                Some(1)
            } else {
                Some(0)
            }
        })
        .sum()
}

/// Check whether two cubes are neighbouring along their third dimension.
fn _neighbours(cube1: &Cube, cube2: &Cube) -> bool {
    let (x1, y1, z1) = &cube1.position;
    let (x2, y2, z2) = &cube2.position;
    if (x1 == x2) & (y1 == y2) & (z2 > z1) {
        z2 - z1 == 1
    } else {
        false
    }
}

pub fn part2() -> i32 {
    0
}
