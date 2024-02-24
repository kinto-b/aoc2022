//! Counting cubes

use std::fs::read_to_string;

// 3D grid ---------------------------------------------------------------------

type Coord = (usize, usize, usize);
struct Grid3d {
    elements: Vec<usize>,
    dim: (usize, usize, usize),
}

impl Grid3d {
    /// Returns the element at location (i, j, k) if it exists
    fn get(&self, coord: Coord) -> Option<&usize> {
        self.elements.get(self._coord_to_idx(coord))
    }

    /// Set the value of the element at location (i, j, k).
    fn set(&mut self, coord: Coord, x: usize) {
        let idx = self._coord_to_idx(coord);
        self.elements[idx] = x;
    }

    /// Convert i,j,k coord to vector index
    fn _coord_to_idx(&self, (i, j, k): Coord) -> usize {
        (i * self.dim.1 * self.dim.2) + (j * self.dim.2) + k
    }
}

// Parse -----------------------------------------------------------------------

fn parse_coord(s: &str) -> Coord {
    let triplet: Vec<usize> = s
        .split_terminator(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    (triplet[0], triplet[1], triplet[2])
}

/// Return the grid and the coordinates of each droplet cube in the grid
fn parse() -> (Grid3d, Vec<Coord>) {
    let input = read_to_string("data/day18.txt").unwrap();
    let cubes: Vec<Coord> = input.lines().map(parse_coord).collect();

    // Collect grid dimensions
    let x = cubes.iter().map(|pt| pt.0).max().unwrap();
    let y = cubes.iter().map(|pt| pt.1).max().unwrap();
    let z = cubes.iter().map(|pt| pt.2).max().unwrap();

    // Put droplet cubes into the grid, leaving some space around the outside
    let elements = vec![0; (x + 3) * (y + 3) * (z + 3)];
    let mut grid = Grid3d {
        elements,
        dim: (x + 3, y + 3, z + 3),
    };

    // The lowest index in the input is 0, but we want to leave some space
    for (x, y, z) in &cubes {
        grid.set((*x + 1, *y + 1, *z + 1), 1);
    }

    (grid, cubes)
}

pub fn part1() -> usize {
    let (grid, cubes) = parse();

    // Ignoring surrounding space, count all the neighbours
    let mut neighbours = 0;
    for i in 1..(grid.dim.0 - 1) {
        for j in 1..(grid.dim.1 - 1) {
            for k in 1..(grid.dim.2 - 1) {
                if grid.get((i, j, k)) == Some(&0) {
                    continue;
                }
                neighbours += sum_neighbours(&grid, (i, j, k))
            }
        }
    }

    (cubes.len() * 6) - neighbours
}

pub fn part2() -> usize {
    let (mut grid, _) = parse();

    // First 'colour' the external cubes
    flood_fill(&mut grid, (0, 0, 0), 2);

    // Next we want to count the neighbours of each *internal* cube
    // Since internal cubes can only neighbour droplet cubes, we can just
    // sum the neighbours.
    let mut neighbours = 0;
    for i in 1..(grid.dim.0 - 1) {
        for j in 1..(grid.dim.1 - 1) {
            for k in 1..(grid.dim.2 - 1) {
                if grid.get((i, j, k)) == Some(&0) {
                    neighbours += sum_neighbours(&grid, (i, j, k))
                }
            }
        }
    }

    part1() - neighbours
}

/// Take the sum of the elements neighbouring a given co-ord
fn sum_neighbours(grid: &Grid3d, coord: Coord) -> usize {
    let (i, j, k) = coord;
    grid.get((i + 1, j, k)).unwrap()
        + grid.get((i - 1, j, k)).unwrap()
        + grid.get((i, j + 1, k)).unwrap()
        + grid.get((i, j - 1, k)).unwrap()
        + grid.get((i, j, k + 1)).unwrap()
        + grid.get((i, j, k - 1)).unwrap()
}

/// Flood the grid with a given value starting from a given position
fn flood_fill(grid: &mut Grid3d, start: Coord, with: usize) {
    if grid.get(start) == Some(&0) {
        grid.set(start, with);
        let (x0, y0, z0) = start;
        flood_fill(grid, (x0 + 1, y0, z0), with);
        flood_fill(grid, (x0.saturating_sub(1), y0, z0), with);
        flood_fill(grid, (x0, y0 + 1, z0), with);
        flood_fill(grid, (x0, y0.saturating_sub(1), z0), with);
        flood_fill(grid, (x0, y0, z0 + 1), with);
        flood_fill(grid, (x0, y0, z0.saturating_sub(1)), with);
    }
}
