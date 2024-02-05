//! Monkey business
//!
//! We can keep our relief manageable in part two by dividing through by
//! the least common multiple (lcm). Fortunately, each monkey has a prime
//! 'divisor' so the lcm is just the product!
//!
//! Probably the distribution of items across the monkeys repeats
//! so there's a decent chance we could speed things up dramatically
//! using caching.

use std::{collections::VecDeque, fs::read_to_string};

// Data class -----------------------------------------------------------------
/// A monkey holding items which it inspects, throws and catches
#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    increment: u64,
    factor: u64,
    power: u32,
    divisor: u64,
    partners: (usize, usize),
    clock: u64,
}

impl Monkey {
    /// Update worry level for each item
    fn update(&mut self, relief: impl Fn(u64) -> u64) {
        for i in 0..self.items.len() {
            self.clock += 1;
            self.items[i] = relief(self.items[i].pow(self.power) * self.factor + self.increment);
        }
    }

    /// Return a tuple containing an item and a usize representing another monkey
    fn throw(&mut self) -> Option<(usize, u64)> {
        let item = self.items.pop_front()?;
        let partner = if item % self.divisor == 0 {
            self.partners.0
        } else {
            self.partners.1
        };

        Some((partner, item))
    }

    /// Add an item to the monkeys items
    fn catch(&mut self, item: u64) {
        self.items.push_back(item)
    }
}

// Parsing --------------------------------------------------------------------
/// Returns a vector of monkeys
fn parse() -> Vec<Monkey> {
    let input = read_to_string("data/day11.txt").unwrap();
    let mut monkeys = Vec::new();

    for block in input.split("\n\r\n") {
        let mut bl = block.lines();
        bl.next(); // Skip "Monkey: " line

        let items = _parse_items(bl.next().unwrap());
        let (increment, factor, power) = _parse_operation(bl.next().unwrap());
        let divisor = _parse_line(bl.next().unwrap());
        let p1 = _parse_line(bl.next().unwrap()) as usize;
        let p2 = _parse_line(bl.next().unwrap()) as usize;

        monkeys.push(Monkey {
            items,
            increment,
            factor,
            power,
            divisor,
            partners: (p1, p2),
            clock: 0,
        })
    }

    monkeys
}

/// Returns a deque containing items
fn _parse_items(l: &str) -> VecDeque<u64> {
    l.replace("Starting items:", "")
        .split(',')
        // .inspect(|x| println!("{x}"))
        .map(|num| num.trim().parse::<u64>().unwrap())
        .collect()
}

/// Returns a tuple representing the increment and factor
fn _parse_operation(l: &str) -> (u64, u64, u32) {
    let add = l.contains('+');
    let pow = l.contains("old * old");

    if pow {
        (0, 1, 2)
    } else {
        let term = _parse_line(l);
        if add {
            (term, 1, 1)
        } else {
            (0, term, 1)
        }
    }
}

/// Returns last word on line, parsed as an integer
fn _parse_line(l: &str) -> u64 {
    l.split_whitespace().last().unwrap().parse::<u64>().unwrap()
}

// Solutions ------------------------------------------------------------------

/// Conduct monkey business, redistributing items across the monkeys
fn monkey_business(monkeys: &mut Vec<Monkey>, rounds: usize, relief: impl Fn(u64) -> u64) {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            monkeys[i].update(&relief);
            while let Some((j, item)) = monkeys[i].throw() {
                monkeys[j].catch(item)
            }
        }
    }
}

/// Returns the level of monkey business after 20 rounds when operating with relief
pub fn part1() -> u64 {
    let mut monkeys = parse();
    monkey_business(&mut monkeys, 20, |x| x / 3);
    monkeys.sort_by(|a, b| b.clock.cmp(&a.clock));
    monkeys[0].clock * monkeys[1].clock
}

/// Returns the level of monkey business after 10,000 rounds when operating without relief
pub fn part2() -> u128 {
    let mut monkeys = parse();

    // monkey divisors are prime, so lcm is product
    let lcm: u64 = monkeys.iter().map(|m| m.divisor).product();

    monkey_business(&mut monkeys, 10_000, |x| x % lcm);
    monkeys.sort_by(|a, b| b.clock.cmp(&a.clock));
    (monkeys[0].clock as u128) * (monkeys[1].clock as u128)
}
