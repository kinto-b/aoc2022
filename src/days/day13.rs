//! Lexicographical ordering
//!
//! Examining the input, we see it is made up of digits 0-10. To ensure
//! that "10" > "9", we convert "10" to ":" which comes after "9" on the
//! ASCII table.
//!
//! To ensure that lists which 'run out' are treated as being smaller than
//! lists which don't we make convert "]" to a symbol which is low on the
//! ASCII table, namely "!".

use std::{cmp::Ordering, fs::read_to_string};

/// Returns the input with the appropriate transformations to ensure
/// proper lexicographical ordering.
fn parse() -> String {
    read_to_string("data/day13.txt")
        .unwrap()
        .replace("10", ":") // Ensure 9 < 10
        .replace(']', "!") // Ensure running out has lowest value
}

/// Return the relative ordering of `left` wrt `right`
fn compare_packets(left: &str, right: &str) -> Ordering {
    let mut left: Vec<char> = left.chars().rev().collect();
    let mut right: Vec<char> = right.chars().rev().collect();

    loop {
        match (left.pop(), right.pop()) {
            (Some('['), Some('[')) => (),

            // If we compare a digit to a list, convert the digit to a
            // singleton list
            (Some('['), Some(b)) => {
                right.push('!');
                right.push(b);
            }
            (Some(a), Some('[')) => {
                left.push('!');
                left.push(a);
            }

            // Otherwise we compare characters directly
            (Some(a), Some(b)) if a < b => {
                return Ordering::Less;
            }
            (Some(a), Some(b)) if b < a => {
                return Ordering::Greater;
            }

            // If one list runs out before the other, it's smaller
            (None, Some(_)) => {
                return Ordering::Less;
            }
            (Some(_), None) => {
                return Ordering::Greater;
            }
            (None, None) => {
                return Ordering::Equal;
            }

            _ => (),
        }
    }
}

/// Returns the number of 'ordered' pairs
pub fn part1() -> i32 {
    let input = parse();
    let mut lists = input.lines().filter(|l| !l.is_empty());

    let mut idx = 0;
    let mut solution = 0;
    while let Some(left) = lists.next() {
        if let Some(right) = lists.next() {
            idx += 1;
            if compare_packets(left, right) == Ordering::Less {
                solution += idx
            }
        }
    }

    solution
}

/// Returns the 'decoder key'
pub fn part2() -> usize {
    let mut input = parse();
    input.push_str("\n\n[[2!!\n[[6!!");

    let mut lists: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();

    lists.sort_by(|&a, &b| compare_packets(a, b));

    let div1 = lists.iter().position(|&x| x == "[[2!!");
    let div2 = lists.iter().position(|&x| x == "[[6!!");

    // Using 1-indexing not 0-indexing
    (div1.unwrap() + 1) * (div2.unwrap() + 1)
}
