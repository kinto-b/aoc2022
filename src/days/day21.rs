//! # Monkey math
//!
//! To solve part one, we simply recurse through the monkeys,
//! caching the computed values as we go. The hardest part about
//! this is satisfying the borrow checker.
//!
//! In theory, this shouldn't work for part two, because we
//! could end up with a complex polynomial in `humn`. Since
//! the solution doesn't help us to identify that polynomial,
//! finding its root would be matter of scanning through
//! (-inf, inf).
//!
//! Fortunately, the polynomial we end up with is of order one.
//! It should, therefore, be possible to come up with a closed
//! form solution using at most three points.
//!
//! I'm at the pub right now, so instead of trying to solve
//! the system of equations simultaneously, I'll just use a
//! binary search algorithm. This works because the effect of
//! a change in `humn` will be monotonic on the output.

use std::{collections::HashMap, fs::read_to_string};

#[derive(Clone)]
struct Monkey<'a> {
    value: Option<f64>,
    formula: Option<(&'a str, &'a str, &'a str)>,
}

impl<'a> Monkey<'a> {
    fn parse(s: &'a str) -> Monkey<'a> {
        let value = s.parse::<f64>();

        match value {
            Ok(x) => Monkey {
                value: Some(x),
                formula: None,
            },
            Err(_) => {
                let symbols: Vec<&str> = s.split_whitespace().collect();
                let formula = (symbols[0], symbols[1], symbols[2]);
                Monkey {
                    value: None,
                    formula: Some(formula),
                }
            }
        }
    }
}

struct MonkeyGang<'a> {
    monkeys: Vec<Monkey<'a>>,
    directory: HashMap<&'a str, usize>,
}

impl MonkeyGang<'_> {
    fn find(&self, name: &str) -> Option<&usize> {
        self.directory.get(name)
    }

    fn eval(&mut self, name: &str) -> f64 {
        let idx = *self.find(name).unwrap();

        let value = match self.monkeys[idx].value {
            None => {
                let (lhs, op, rhs) = self.monkeys[idx].formula.unwrap();
                combine(op, self.eval(lhs), self.eval(rhs))
            }
            Some(x) => x,
        };

        self.monkeys[idx].value = Some(value); // Cache the value
        value
    }
}

fn combine(op: &str, x: f64, y: f64) -> f64 {
    match op {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        _ => unreachable!(),
    }
}

fn parse(input: &'_ str) -> MonkeyGang<'_> {
    let mut directory: HashMap<&str, usize> = HashMap::new();
    let mut monkeys = Vec::new();

    for (i, l) in input.lines().enumerate() {
        let (name, shout) = l.split_once(':').unwrap();
        let monkey = Monkey::parse(shout.trim());
        monkeys.push(monkey);
        directory.insert(name, i);
    }

    MonkeyGang { monkeys, directory }
}

pub fn part1() -> f64 {
    let input = read_to_string("data/day21.txt").unwrap();
    let mut monkeys = parse(&input);

    monkeys.eval("root")
}

pub fn part2() -> f64 {
    let input = read_to_string("data/day21.txt").unwrap();
    let mut gang = parse(&input);

    // Corrections
    let iroot = *gang.find("root").unwrap();
    let (lhs, _, rhs) = gang.monkeys[iroot].formula.unwrap();
    gang.monkeys[iroot] = Monkey {
        value: None,
        formula: Some((lhs, "-", rhs)),
    };

    // Record monkeys
    let monkeys0 = gang.monkeys.clone();

    // First find boundaries to search within
    let x1 = 0.0;
    let y1 = test(&mut gang, x1);

    let mut delta = 1_000_000.0;
    let mut x2 = x1 + delta;
    gang.monkeys = monkeys0.clone();
    let mut y2 = test(&mut gang, x2);

    if y2.abs() > y1.abs() {
        delta *= -1.0; // Wrong way, go back!
    }

    while y2.signum() == y1.signum() {
        gang.monkeys = monkeys0.clone();
        x2 += delta;
        y2 = test(&mut gang, x2);
        delta *= 2.0;
    }

    // Now binary search within bounds
    let mut lwr = if y1 < y2 { x1 } else { x2 };
    let mut upr = if y1 < y2 { x2 } else { x1 };
    loop {
        gang.monkeys = monkeys0.clone();
        let xx = (lwr + upr) / 2.0;
        let yy = test(&mut gang, xx);

        match yy {
            b if b > 0.0 => upr = xx,
            b if b < 0.0 => lwr = xx,
            _ => return xx,
        }
    }
}

fn test(gang: &mut MonkeyGang, humn: f64) -> f64 {
    let ihumn = *gang.find("humn").unwrap();
    gang.monkeys[ihumn] = Monkey {
        value: Some(humn),
        formula: None,
    };

    gang.eval("root")
}
