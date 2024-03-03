//! # Monkey math
//!
//! To solve part one, we simply recurse through the monkeys,
//! caching the computed values as we go. The hardest part about
//! this is satisfying the borrow checker.
//!
//! Originally I solved part two using a binary search, which was
//! fairly quick, but we can do even better. Since each of the
//! computation involves exactly two terms, we have a binary tree.
//! And since `humn` is referenced only once, at each node in the
//! tree, at most one of the branches can include `humn`, which
//! is the only unknown. So we can divide and conquer! To balance
//! the root, we need lhs0-rhs0=0. If `humn` is in the rhs, then
//! lhs is known, and so we know the value we need for `rhs`. To
//! balance `rhs`, we need lhs1-rhs1=lhs0. &c.

use std::{collections::HashMap, fs::read_to_string};

#[derive(Clone, Debug)]
struct Monkey<'a> {
    value: Option<i64>,
    formula: Option<(&'a str, &'a str, &'a str)>,
}

impl<'a> Monkey<'a> {
    fn parse(s: &'a str) -> Monkey<'a> {
        let value = s.parse::<i64>();

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
    /// Return the value of the monkey with the provided name
    fn eval(&mut self, name: &str) -> i64 {
        let idx = self._find(name);

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

    /// Return the value of `humn` needed to balance the root node.
    fn balance(&mut self) -> i64 {
        self._reset_humn();

        let iroot = self._find("root");
        let (lhs, _, rhs) = self.monkeys[iroot].formula.unwrap();
        self.monkeys[iroot] = Monkey {
            value: None,
            formula: Some((lhs, "-", rhs)),
        };

        self._balance_node("root", 0)
    }

    fn _balance_node(&mut self, name: &str, z: i64) -> i64 {
        let idx = self._find(name);

        match self.monkeys[idx].formula {
            Some((lhs, op, rhs)) => {
                let ldx = self._find(lhs);
                let rdx = self._find(rhs);

                if let Some(x) = self.monkeys[ldx].value {
                    self._balance_node(rhs, solve_for_y(op, z, x))
                } else if let Some(y) = self.monkeys[rdx].value {
                    self._balance_node(lhs, solve_for_x(op, z, y))
                } else {
                    unreachable!()
                }
            }
            None => z,
        }
    }

    /// Set to None the values of all nodes in the branch that leads to `humn`
    fn _reset_humn(&mut self) {
        let humn_idx = self._find("humn");
        self._reset_node("root", humn_idx);
    }

    /// Check if a node is in a branch leading to `humn` and set its value to None if so
    fn _reset_node(&mut self, name: &str, humn_idx: usize) -> bool {
        let idx = self._find(name);

        let mut reset = idx == humn_idx;

        if let Some((lhs, _, rhs)) = self.monkeys[idx].formula {
            reset = self._reset_node(lhs, humn_idx) || self._reset_node(rhs, humn_idx)
        }

        if reset {
            self.monkeys[idx].value = None;
        }

        reset
    }

    /// Return the index of the monkey with the given name
    fn _find(&self, name: &str) -> usize {
        *self.directory.get(name).unwrap()
    }
}

fn combine(op: &str, x: i64, y: i64) -> i64 {
    match op {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        _ => unreachable!(),
    }
}

fn solve_for_x(op: &str, z: i64, y: i64) -> i64 {
    match op {
        "+" => z - y,
        "-" => z + y,
        "*" => z / y,
        "/" => z * y,
        _ => unreachable!(),
    }
}
fn solve_for_y(op: &str, z: i64, x: i64) -> i64 {
    match op {
        "+" => z - x,
        "-" => x - z,
        "*" => z / x,
        "/" => x / z,
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

pub fn part1() -> i64 {
    let input = read_to_string("data/day21.txt").unwrap();
    let mut monkeys = parse(&input);
    monkeys.eval("root")
}

pub fn part2() -> i64 {
    let input = read_to_string("data/day21.txt").unwrap();
    let mut gang = parse(&input);
    gang.eval("root");
    gang.balance()
}
