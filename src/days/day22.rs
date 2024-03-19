//! # Monkey map
//!
//! This was a tough puzzle! I had to look at some solutions online before I
//! could work out part two. There are three key ideas involved in the solution.
//!
//! The first is using 90 degree rotations of a 3d coordinate system to keep
//! track of the cube faces. Imagine standing in the starting position, facing
//! right. We set up our coordinate system so that the first dimension points
//! back behind us, the second dimension points to our left, and the third
//! dimension points directly up, from our feet towards our head. If we walk
//! down far enough, we'll go off the edge of the first face. Then we'll need
//! to rotate clockwise about the first axis. Likewise if we walk north off
//! our current face, we'll rotate counterclockwise about the first axis; if
//!  we walk left, clockwise about the second axis; if right, counterclockwise
//! about the second axis.
//!
//! We use a breadth first search to locate each cube face on the 2D grid,
//! associating each rotation with a single point corresponding to the top-left
//! corner of that face. Once we've done that, we know which 50x50 subgrid we'll
//! end up at if we walk off of a given face in any direction.
//!
//! The second key idea involved in the solution of part two is to recognise
//! that we can figure out which edge of the subgrid (top, bottom, left or right)
//! we'll be on by comparing the state of the coordinate system before and
//! after we moved to the new cube face. If we could return to the previous
//! face via a clockwise rotation about the first axis, equivalent to a move
//! down in 2D space, we must be on the bottom of the new face.
//!
//! The final key idea is that the we can determine our precise location on the
//! face/edge we've arrived at simply by considering the location we departed from,
//! the edge (top, bottom, left, right) we departed from, and the edge we arrived
//! at.
//!
//! The solution to part one is complicated by the fact that the components of
//! the solution are reused in part two.

use std::{collections::HashMap, fs::read_to_string, ops::Neg};

use crate::{grid::Grid, parse::parse_u32};

const SIDE_LENGTH: i32 = 50; // Side-length of each cube face

// Data classes ---------------------------------------------------------------

type Position = (i32, i32);
type Map = Grid<char>;
type Instructions = (Vec<u32>, Vec<char>);

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(self) -> Direction {
        match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        }
    }
    fn turn_right(self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn score(&self) -> i32 {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Vector {
    i: i32,
    j: i32,
    k: i32,
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector {
            i: -self.i,
            j: -self.j,
            k: -self.k,
        }
    }
}

#[derive(Clone, Copy)]
struct Face {
    i: Vector,
    j: Vector,
    k: Vector,
    corner: (i32, i32),
}

#[derive(Clone, Copy)]
struct State {
    offset: Position,
    face: Face,
    dir: Direction,
}

impl State {
    fn grid_position(&self) -> Position {
        (
            self.offset.0 + self.face.corner.0,
            self.offset.1 + self.face.corner.1,
        )
    }

    fn score(&self) -> i32 {
        let (x, y) = self.grid_position();
        1000 * (y + 1) + 4 * (x + 1) + self.dir.score()
    }
}

// Parse ----------------------------------------------------------------------
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

    (map, (start_col as i32, 0), (steps, turns))
}

pub fn part1() -> i32 {
    let (grid, start, instructions) = parse();
    let face = Face {
        i: Vector { i: 1, j: 0, k: 0 },
        j: Vector { i: 0, j: 1, k: 0 },
        k: Vector { i: 0, j: 0, k: 1 },
        corner: start,
    };

    let state = State {
        offset: (0, 0),
        face,
        dir: Direction::Right,
    };

    let teleporter = |state: State| -> State {
        let mut pos = state.grid_position();
        let mut face = state.face;

        loop {
            pos = step_wrap(&pos, &state.dir, (grid.ncol as i32, grid.nrow as i32));
            match tile(pos, &grid) {
                Some('.') => break,
                Some('#') => break,
                _ => (),
            }
        }

        let (x, y) = pos;
        let offset = (x % 50, y % 50);
        face.corner = (50 * (x / 50), 50 * (y / 50));

        State {
            offset,
            face,
            dir: state.dir,
        }
    };

    password(state, &grid, &instructions, &teleporter)
}

pub fn part2() -> i32 {
    let (grid, start, instructions) = parse();
    let face = Face {
        i: Vector { i: 1, j: 0, k: 0 },
        j: Vector { i: 0, j: 1, k: 0 },
        k: Vector { i: 0, j: 0, k: 1 },
        corner: start,
    };
    let faces = locate_faces(face, &grid);

    let state = State {
        offset: (0, 0),
        face,
        dir: Direction::Right,
    };

    let teleporter = |state: State| -> State {
        let State { offset, face, dir } = state;
        let Face { i, j, k, corner: _ } = face;

        let next_k = match dir {
            Direction::Down => -j,
            Direction::Up => j,
            Direction::Left => i,
            Direction::Right => -i,
        };

        let next_face = faces.get(&next_k).unwrap();

        let next_dir = if k == next_face.j {
            Direction::Down
        } else if k == -next_face.j {
            Direction::Up
        } else if k == -next_face.i {
            Direction::Left
        } else if k == next_face.i {
            Direction::Right
        } else {
            unreachable!()
        };

        let edge = SIDE_LENGTH - 1;
        let next_offset = match (dir, next_dir) {
            (Direction::Down, Direction::Down) => (offset.0, 0),
            (Direction::Down, Direction::Up) => (edge - offset.0, edge),
            (Direction::Down, Direction::Left) => (edge, offset.0),
            (Direction::Down, Direction::Right) => (0, edge - offset.0),

            (Direction::Up, Direction::Down) => (edge - offset.0, 0),
            (Direction::Up, Direction::Up) => (offset.0, edge),
            (Direction::Up, Direction::Left) => (edge, edge - offset.0),
            (Direction::Up, Direction::Right) => (0, offset.0),

            (Direction::Left, Direction::Down) => (offset.1, 0),
            (Direction::Left, Direction::Up) => (edge - offset.1, edge),
            (Direction::Left, Direction::Left) => (edge, offset.1),
            (Direction::Left, Direction::Right) => (0, edge - offset.1),

            (Direction::Right, Direction::Down) => (edge - offset.1, 0),
            (Direction::Right, Direction::Up) => (offset.1, edge),
            (Direction::Right, Direction::Left) => (edge, edge - offset.1),
            (Direction::Right, Direction::Right) => (0, offset.1),
        };

        State {
            offset: next_offset,
            face: *next_face,
            dir: next_dir,
        }
    };

    password(state, &grid, &instructions, &teleporter)
}

// Helpers --------------------------------------------------------------------

/// Return the tile at the given location in the grid
fn tile((x, y): (i32, i32), grid: &Grid<char>) -> Option<char> {
    if x < 0 || x >= (grid.ncol as i32) || y < 0 || y >= (grid.nrow as i32) {
        return None;
    }

    Some(grid.get(y as usize, x as usize))
}

/// Return the password computed after walking the grid
fn password(
    state: State,
    grid: &Grid<char>,
    instructions: &Instructions,
    teleporter: &impl Fn(State) -> State,
) -> i32 {
    let mut state = state;
    let mut turns = instructions.1.iter();
    for &steps in &instructions.0 {
        state = walk(state, steps, grid, teleporter);
        match turns.next() {
            Some('R') => state.dir = state.dir.turn_right(),
            Some('L') => state.dir = state.dir.turn_left(),
            _ => (),
        }
    }
    state.score()
}

/// Returns the location after taking one step
fn step((x, y): &Position, dir: &Direction) -> Position {
    match dir {
        Direction::Down => (*x, y + 1),
        Direction::Up => (*x, y - 1),
        Direction::Right => (x + 1, *y),
        Direction::Left => (x - 1, *y),
    }
}

/// Returns the location after taking one step, wrapping around the boundary
fn step_wrap(p: &Position, dir: &Direction, (bx, by): Position) -> Position {
    let (mut x, mut y) = step(p, dir);

    if x < 0 {
        x = bx - 1;
    } else if x >= bx {
        x = 0
    }

    if y < 0 {
        y = by - 1;
    } else if y >= by {
        y = 0
    }

    (x, y)
}

fn walk(state: State, steps: u32, grid: &Grid<char>, teleporter: impl Fn(State) -> State) -> State {
    let mut state = state;
    for _ in 0..steps {
        let mut next_state = state;
        next_state.offset = step(&state.offset, &state.dir);

        let (x, y) = next_state.offset;
        if !(0..50).contains(&x) || !(0..50).contains(&y) {
            next_state = teleporter(state);
        }

        match tile(next_state.grid_position(), grid) {
            Some('.') => {
                state = next_state;
            }
            Some('#') => {
                break;
            }
            _ => unreachable!(),
        }
    }

    state
}

// Return all six cube Faces, finding them via BFS
fn locate_faces(start: Face, grid: &Grid<char>) -> HashMap<Vector, Face> {
    let mut queue = vec![start];
    let mut faces = HashMap::new();

    while let Some(Face { i, j, k, corner }) = queue.pop() {
        let adjacent = [
            // Down
            Face {
                i,
                j: k,
                k: -j,
                corner: (corner.0, corner.1 + 50),
            },
            // Up
            Face {
                i,
                j: -k,
                k: j,
                corner: (corner.0, corner.1 - 50),
            },
            // Left
            Face {
                i: -k,
                j,
                k: i,
                corner: (corner.0 - 50, corner.1),
            },
            // Right
            Face {
                i: k,
                j,
                k: -i,
                corner: (corner.0 + 50, corner.1),
            },
        ];

        for adj in adjacent {
            if faces.contains_key(&adj.k) {
                continue;
            }

            match tile(adj.corner, grid) {
                Some('.') => {
                    faces.insert(adj.k, adj);
                    queue.push(adj);
                }
                Some('#') => {
                    faces.insert(adj.k, adj);
                    queue.push(adj);
                }
                _ => continue,
            }
        }
    }

    faces
}
