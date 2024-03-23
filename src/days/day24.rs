//! # Blizzard Basin
//!
//! I had two initial thoughts when I first read this problem description. The
//! first was 'BFS' and the second was 'bit shift'.
//!
//! Ignoring the vertical moving blizzards for the moment, if we represent each
//! row in the basin as a 128 bit integer where the blizzards are on bits, we
//! can simulate one minute by bit shifting left or right. It'll be simplest
//! to keep the left- and right- blizzards separated, so we'll use two 128 bit
//! integers per row, shifting one left and the other right each minute.
//!
//! Since the blizzards cycle, we'll return to the start after W minutes, where
//! W is the width of the basin. Therefore, we'll be able to store every
//! possible state of the basin using just 2*H*W 128 bit integers.
//!
//! We can do the same thing with up- and down- blizzards, except using 128 bit
//! integers to represent *columns* instead of rows.
//!
//! There's just one detail left to figure out. How do we get the wrapping to
//! work? We can't just 'rotate' the bits because there are fewer rows/cols
//! in the basin than bits in the integer. So let's think. If we start at
//! position P and move D steps forward we'll end up at either P+D
//! or P+D-N = P-(N-D), depending on whether P+D<N (i.e. P<N-D) or not. So
//! suppose we shifted forward by D *and* backward by (N-D) and then took a
//! bitwise OR. Either P+D<N or not,
//!
//!   - If P+D<N, then P<N-D, so shifting back by N-D shifts the bit out of
//!     the integer. Thus the OR just gives us the result of the forward shift,
//!     which is what we wanted.
//!
//!   - If P+D>=N, then the forward shift sends the bit out of the integer (or
//!     at least out of the part we care about), so the OR just gives us the
//!     result of the backward shift, which is what we want.
//!
//! Ok, now let's implement it!

use std::fs::read_to_string;

fn parse() -> (Vec<Vec<i128>>, Vec<Vec<i128>>) {
    let input = read_to_string("data/day24.txt").unwrap();
    let width = input.lines().next().unwrap().len() - 2; // 2 border chars
    let height = input.lines().count() - 2; // 2 border chars

    let basin = input.lines().skip(1).take(height).map(|l| &l[1..width]);

    // Starting state
    let mut left = vec![0; height];
    let mut right = vec![0; height];
    let mut up = vec![0; width];
    let mut down = vec![0; width];

    for (i, row) in basin.enumerate() {
        for (j, ch) in row.bytes().enumerate() {
            match ch {
                b'<' => left[i] |= 1 >> j,
                b'>' => right[i] |= 1 >> j,
                b'^' => up[j] |= 1 >> i,
                b'v' => down[j] |= 1 >> i,
                _ => (),
            }
        }
    }

    // Compute all possible basin states
    let mut row_states = Vec::with_capacity(width);
    for t in 0..width {
        let mut state = Vec::with_capacity(height);
        for i in 0..height {
            let left = left[i] << t | left[i] >> (width - t);
            let right = right[i] >> t | right[i] << (width - t);
            state.push(left | right);
        }
        row_states.push(state);
    }

    let mut col_states = Vec::with_capacity(height);
    for t in 0..height {
        let mut state = Vec::with_capacity(width);
        for i in 0..width {
            let up = up[i] << t | up[i] >> (height - t);
            let down = down[i] >> t | down[i] << (height - t);
            state.push(up | down);
        }
        col_states.push(state);
    }

    (row_states, col_states)
}

pub fn part1() -> i32 {
    let (row_states, col_states) = parse();
    // TBC, I have to go look after my niece!
    0
}

pub fn part2() -> i32 {
    0
}
