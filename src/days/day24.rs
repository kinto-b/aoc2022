//! # Blizzard Basin
//!
//! I had two initial thoughts when I first read this problem description. The
//! first was 'BFS' and the second was 'bit shift'.
//!
//! Ignoring the vertically moving blizzards for the moment, if we represent each
//! row in the basin as a 128 bit integer where the blizzards are on bits, we
//! can simulate one minute by bit shifting left or right. It'll be simplest
//! to keep the left- and right- blizzards separated, so we'll use two 128 bit
//! integers per row, shifting one left and the other right each minute.
//!
//! Since the blizzards cycle, we'll return to the start after W minutes, where
//! W is the width of the basin. Therefore, we'll be able to store every
//! possible state of the basin using just 2*H*W 128 bit integers.
//!
//! There's just one detail left to figure out. How do we get the wrapping to
//! work? We can't just 'rotate' the bits because there are fewer rows/cols
//! in the basin than bits in the integer. So let's think. If we start at
//! position P and move D steps forward we'll end up at either P+D
//! or P+D-W = P-(W-D), depending on whether P+D<W (i.e. P<W-D) or not. So
//! suppose we shifted forward by D *and* backward by (W-D) and then took a
//! bitwise OR. Either P+D<W or not,
//!
//!   - If P+D<W, then P<W-D, so shifting back by W-D shifts the bit out of
//!     the integer. Thus the OR just gives us the result of the forward shift,
//!     which is what we wanted.
//!
//!   - If P+D>=W, then the forward shift sends the bit out of the integer (or
//!     at least out of the part we care about), so the OR just gives us the
//!     result of the backward shift, which is what we want.
//!
//! OK, now what about vertically moving blizzards? Initially I thought to do
//! the same thing, just using integers to represent cols instead of rows. But
//! this makes it hard to figure out which grid locations are free of any
//! blizzards. Instead we'll use integers to represent rows again, but instead
//! of using bit shifts to represent movements, we'll use indexing. After D
//! minutes, the downward moving blizzards initially in row P will end up in
//! row (P+D)%H, where H is the height of the grid.

use std::fs::read_to_string;

struct Basin {
    height: usize,
    width: usize,
    free_rows: Vec<Vec<u128>>,
    free_cols: Vec<Vec<u128>>,
}

fn parse() -> Basin {
    let input = read_to_string("data/day24.txt").unwrap();
    let width = input.lines().next().unwrap().len() - 2; // 2 border chars
    let height = input.lines().count() - 2; // 2 border chars

    let basin = input
        .lines()
        .skip(1)
        .take(height)
        .map(|l| &l[1..(width + 1)]);

    // Starting state
    let mut left = vec![0; height];
    let mut right = vec![0; height];
    let mut up = vec![0; width];
    let mut down = vec![0; width];

    for (i, row) in basin.enumerate() {
        for (j, ch) in row.bytes().enumerate() {
            match ch {
                b'<' => left[i] |= 1 << j,
                b'>' => right[i] |= 1 << j,
                b'^' => up[i] |= 1 << j,
                b'v' => down[i] |= 1 << j,
                _ => (),
            }
        }
    }

    // Compute all possible basin states. Flip the bits so
    // free spots are marked.
    let mut free_rows = Vec::with_capacity(width);
    for t in 0..width {
        let mut state = Vec::with_capacity(height);
        for i in 0..height {
            let left = left[i] >> t | left[i] << (width - t);
            let right = right[i] << t | right[i] >> (width - t);
            state.push(!left & !right);
        }
        free_rows.push(state);
    }

    let mut free_cols = Vec::with_capacity(height);
    for t in 0..height {
        let mut state = Vec::with_capacity(height);
        for i in 0..height {
            let up = up[(i + t) % height];
            let down = down[(height + i - t % height) % height];
            state.push(!up & !down);
        }
        free_cols.push(state);
    }

    Basin {
        height,
        width,
        free_rows,
        free_cols,
    }
}

pub fn part1() -> usize {
    let basin = parse();
    let start = (0, 1); // First row, first bit
    let end = (basin.height - 1, 1 << (basin.width - 1)); // Last row, last bit

    bfs(start, end, 0, &basin)
}

pub fn part2() -> usize {
    let basin = parse();
    let start = (0, 1); // First row, first bit
    let end = (basin.height - 1, 1 << (basin.width - 1)); // Last row, last bit

    let leg1 = bfs(start, end, 0, &basin);
    let leg2 = bfs(end, start, leg1, &basin);
    bfs(start, end, leg2, &basin)
}

fn bfs(start: (usize, u128), end: (usize, u128), time: usize, basin: &Basin) -> usize {
    let Basin {
        height,
        width,
        free_rows,
        free_cols,
    } = basin;
    let mut t = time;

    // Now at each moment in time, we populate every reachable location that
    // is free by looping over the rows and setting each bit to on which has
    // a neighbouring on bit and which is not occupied by a blizzard.
    let mut elf = vec![0; height + 1]; // +1 to prevent index OOB

    loop {
        let mut elf_row = 0;
        for i in 0..*height {
            // The row at [i-1] was modified in the last iteration, so we
            // need to store the original value each time.
            let elf_up = elf_row;
            elf_row = elf[i];
            let elf_down = elf[i + 1];

            elf[i] = (elf_row | elf_row << 1 | elf_row >> 1 | elf_down | elf_up)
                & free_rows[t % width][i]
                & free_cols[t % height][i];
        }

        if elf[end.0] & end.1 > 0 {
            return t + 1;
        }

        elf[start.0] |= start.1; // We can wait at the start for as long as we need.
        t += 1;
    }
}
