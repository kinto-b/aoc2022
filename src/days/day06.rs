//! Slide a window along the stream and check that the characters are unique 
//! within the window.
//! 
//! TODO: This is somewhat wasteful since we effectively check each element
//! as many times as the window is wide. It would be better to do something like
//! 
//! window = []
//! if i := window.locate(x):
//!     window = window[(i+1):] + [x]
//! else:
//!     window += [x]
//! 
//! and stop when window.len() == threshold.

/// Returns true if all characters in the buffer are unique
fn is_marker(buffer: &[u8]) -> bool {
    let n = buffer.len() as u32;
    n == buffer.iter()
        .map(|i| 1 << (i - b'a' + 1))
        .fold(0, |acc: u32, i: u32| acc | i )
        .count_ones()
}

pub fn part1() -> usize {
    4 + include_bytes!("../../data/day06.txt")
        .windows(4)
        .position(is_marker)
        .unwrap()
}

pub fn part2() -> usize {
    14 + include_bytes!("../../data/day06.txt")
        .windows(14)
        .position(is_marker)
        .unwrap()
}