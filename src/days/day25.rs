//! # Full of Hot Air
//!
//! An easy problem to finish :)

use std::fs::read_to_string;

pub fn part1() -> String {
    let input = read_to_string("data/day25.txt").unwrap();

    let x: i64 = input.lines().map(from_snafu).sum();
    to_snafu(x)
}

pub fn part2() -> i64 {
    0
}

fn from_snafu(x: &str) -> i64 {
    let n = x.len() - 1;
    x.chars()
        .map(|ch| match ch {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => unreachable!(),
        })
        .enumerate()
        .map(|(p, i)| i * 5_i64.pow((n - p) as u32))
        .sum()
}

fn to_snafu(x: i64) -> String {
    let mut snafu = String::new();
    let (mut d, mut m) = (x / 5, x % 5);
    while d + m > 0 {
        match m {
            0 => snafu.push('0'),
            1 => snafu.push('1'),
            2 => snafu.push('2'),
            3 => {
                snafu.push('=');
                d += 1;
            }
            4 => {
                snafu.push('-');
                d += 1;
            }
            _ => unreachable!(),
        }

        (d, m) = (d / 5, d % 5)
    }

    snafu.chars().rev().collect()
}
