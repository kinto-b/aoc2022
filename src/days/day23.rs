//! # Unstable Diffusion
//!
//! After the terror of day 22, today's problem was mercifully easy to solve.
//! However my solution is pretty slow when it comes to part two. It takes
//! about 4 seconds on my dinky laptop.
//!
//! It's 11pm so I'm not going to bother optimising it, but a couple of
//! ideas come directly to mind. For instance, once an elf has stopped
//! moving, it'll only start again if another elf approaches it. So
//! we should be able to save a bunch of computations by excluding
//! elves when they stop. We reintroduce them when needed by checking
//! for new neighbours of moving elves.

use std::fs::read_to_string;

use crate::grid::Grid;

type Position = (i32, i32);

const MARGIN: usize = 100;

// N, S, W, E
const PROPOSALS: [Position; 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

const CONDITIONS: [[Position; 3]; 4] = [
    [(-1, -1), (0, -1), (1, -1)], // NW, N, NE
    [(-1, 1), (0, 1), (1, 1)],    // SW, S, SE
    [(-1, -1), (-1, 0), (-1, 1)], // NW, W, SW
    [(1, -1), (1, 0), (1, 1)],    // NE, E, SE
];

const NEIGHBOURS: [Position; 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn parse() -> (Vec<Position>, Grid<char>) {
    let input = read_to_string("data/day23.txt").unwrap();
    let mut elves = Vec::new();

    let grid_width = input.lines().next().unwrap().len() + 2 * MARGIN;
    let mut grid = Grid::new(vec!['.'; grid_width * grid_width], grid_width);

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                elves.push(((x + MARGIN) as i32, (y + MARGIN) as i32));
                grid.set(y + MARGIN, x + MARGIN, '#');
            }
        }
    }

    (elves, grid)
}

pub fn part1() -> i32 {
    let (mut elves, mut grid) = parse();

    for step in (0..4).cycle().take(10) {
        let mut proposals = gather_proposals(&elves, &grid, step);
        resolve_proposals(&mut proposals, &elves);
        exectue_proposals(&proposals, &mut elves, &mut grid);
    }

    let x: Vec<i32> = elves.iter().map(|(x, _)| *x).collect();
    let y: Vec<i32> = elves.iter().map(|(_, y)| *y).collect();

    let width = x.iter().max().unwrap() - x.iter().min().unwrap();
    let height = y.iter().max().unwrap() - y.iter().min().unwrap();

    (height + 1) * (width + 1) - (elves.len() as i32)
}

pub fn part2() -> i32 {
    let (mut elves, mut grid) = parse();

    let mut s = 0;
    for step in (0..4).cycle() {
        s += 1;
        let mut proposals = gather_proposals(&elves, &grid, step);
        resolve_proposals(&mut proposals, &elves);
        let any_moved = exectue_proposals(&proposals, &mut elves, &mut grid);

        if !any_moved {
            break;
        }
    }

    s
}

/// Return a grid containing movement proposals for each elf. The proposal at (i,j) in this grid
/// is for the elf at (i,j) in the elf grid.
fn gather_proposals(
    elves: &Vec<Position>,
    grid: &Grid<char>,
    step: usize,
) -> Grid<Option<Position>> {
    let mut proposals: Grid<Option<Position>> = Grid::new(vec![None; grid.len()], grid.nrow);
    for i in (0..4).cycle().skip(step).take(4) {
        let condition = CONDITIONS[i];

        for (ex, ey) in elves {
            if proposals.get(*ey as usize, *ex as usize).is_some() {
                continue; // Already recieved a proposal
            };

            let elf_moves = count_free((*ex, *ey), &NEIGHBOURS, grid) < 8;
            let proposal_ok = count_free((*ex, *ey), &condition, grid) == condition.len();

            if proposal_ok && elf_moves {
                proposals.set(*ey as usize, *ex as usize, Some(PROPOSALS[i]));
            }
        }
    }

    proposals
}

/// Return the number of free squares among the neighbours defined by `delta`
fn count_free((x, y): Position, delta: &[Position], grid: &Grid<char>) -> usize {
    delta
        .iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .filter(|(x, y)| grid.get(*y as usize, *x as usize) == '.')
        .count()
}

/// Remove clashing proposals from the proposal grid
fn resolve_proposals(proposals: &mut Grid<Option<Position>>, elves: &Vec<Position>) {
    for (ex, ey) in elves {
        if let Some((px, py)) = proposals.get(*ey as usize, *ex as usize) {
            // Each elf only needs to cross-check the proposals of 3 competitors
            let competitors: [Position; 3] = if px == 0 {
                [(*ex, ey + 2 * py), (ex - 1, ey + py), (ex + 1, ey + py)]
            } else {
                [(ex + 2 * px, *ey), (ex + px, ey - 1), (ex + px, ey + 1)]
            };

            let mut spoiled = false;
            for (cx, cy) in competitors {
                if let Some((cpx, cpy)) = proposals.get(cy as usize, cx as usize) {
                    if (ex + px, ey + py) == (cx + cpx, cy + cpy) {
                        proposals.set(cy as usize, cx as usize, None);
                        spoiled = true;
                    }
                }
            }

            if spoiled {
                proposals.set(*ey as usize, *ex as usize, None);
            }
        }
    }
}

/// Execute proposals, moving the elves on the grid accordingly
fn exectue_proposals(
    proposals: &Grid<Option<Position>>,
    elves: &mut [Position],
    grid: &mut Grid<char>,
) -> bool {
    let mut moved = false;
    for e in elves.iter_mut() {
        let (ex, ey) = *e;
        if let Some((px, py)) = proposals.get(ey as usize, ex as usize) {
            let (nx, ny) = (ex + px, ey + py);
            grid.set(ey as usize, ex as usize, '.');
            grid.set(ny as usize, nx as usize, '#');
            *e = (ex + px, ey + py);
            moved = true;
        }
    }

    moved
}
