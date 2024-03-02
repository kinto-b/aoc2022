//! Collecting minerals
//!
//! This is another branch and bound problem. The trickiest bit is coming up
//! with a decent upper bound. So consider
//!
//! - We cannot possibly do better than we could if we were ignoring ore.
//! - We cannot possibly have more clay than T+2(T-1)+3(T-2)+... less the amount
//! we have spent on obsidian robots. So we'll just assume we always have such
//! an amount
//! - And so, given we're assuming maximal clay balance, we cannot possibly get
//! more obsidian than we could by building obsidian bots whenever we can.
//! - And so, given we're assuming maximal obsidian balance, we cannot possibly
//! get more geodes than we could by building geode bots whenever we can.
//!
//! We also don't need to mine more resources in a given turn than we can spend
//! in a given turn, so we can prune branches where the number of robots of
//! a given type exceeds the amount of that resource we can spend in one turn.

use std::fs::read_to_string;

use crate::parse::parse_u32;

// Data classes ----------------------------------------------------------------
type Minerals = [u8; 4];

fn add(x: &Minerals, y: &Minerals) -> Minerals {
    let mut z: Minerals = [0; 4];
    for ((zz, xx), yy) in z.iter_mut().zip(x).zip(y) {
        *zz = xx.saturating_add(*yy); // We'll never have >256 minerals
    }
    z
}

fn sub(x: &Minerals, y: &Minerals) -> Minerals {
    let mut z: Minerals = [0; 4];
    for ((zz, xx), yy) in z.iter_mut().zip(x).zip(y) {
        *zz = xx - yy;
    }
    z
}

fn le(x: &Minerals, y: &Minerals) -> bool {
    x.iter().zip(y).all(|(xx, yy)| xx <= yy)
}

struct State {
    balance: Minerals,
    robots: Minerals,
    countdown: u8,
}

struct Blueprint {
    costs: [Minerals; 4],
    limits: Minerals, // The most we'll spend of each mineral in a given minute
}

impl Blueprint {
    fn parse(s: &str) -> Self {
        let integers = parse_u32(s);
        let mut ints = integers.iter();

        let mut costs: [Minerals; 4] = [[0; 4]; 4];
        ints.next(); // First element is blueprint index
        costs[0][0] = *ints.next().unwrap() as u8;
        costs[1][0] = *ints.next().unwrap() as u8;
        costs[2][0] = *ints.next().unwrap() as u8;
        costs[2][1] = *ints.next().unwrap() as u8;
        costs[3][0] = *ints.next().unwrap() as u8;
        costs[3][2] = *ints.next().unwrap() as u8;

        Blueprint::new(costs)
    }

    fn new(costs: [Minerals; 4]) -> Self {
        let mut limits = [0; 4];
        for i in 0..4 {
            limits[i] = costs.iter().map(|x| x[i]).max().unwrap();
        }
        limits[3] = u8::MAX; // Don't limit geode bots

        Blueprint { costs, limits }
    }
}

// Solution --------------------------------------------------------------------

fn parse() -> Vec<Blueprint> {
    let input = read_to_string("data/day19.txt").unwrap();
    input.lines().map(Blueprint::parse).collect()
}

pub fn part1() -> usize {
    let blueprints = parse();
    let scores = blueprints.iter().map(|blueprint| {
        let state = State {
            balance: [0; 4],
            robots: [1, 0, 0, 0],
            countdown: 24,
        };
        branch_and_bound(state, blueprint, 0)
    });

    scores
        .enumerate()
        .map(|(i, s)| (i + 1) * (s as usize))
        .sum()
}

pub fn part2() -> usize {
    let blueprints = parse();
    let scores = blueprints.iter().take(3).map(|blueprint| {
        let state = State {
            balance: [0; 4],
            robots: [1, 0, 0, 0],
            countdown: 32,
        };
        branch_and_bound(state, blueprint, 0)
    });

    scores.map(|i| i as usize).product()
}

fn branch_and_bound(state: State, blueprint: &Blueprint, best: u8) -> u8 {
    let State {
        mut balance,
        robots,
        mut countdown,
    } = state;

    // Check upper bound
    let mut best = best.max(balance[3]);
    if upper_bound(state, blueprint) <= best {
        return best;
    }

    while countdown > 0 {
        // Build a new robot
        for (i, cost) in blueprint.costs.iter().enumerate() {
            if !le(cost, &balance) || (robots[i] >= blueprint.limits[i]) {
                // Don't build a robot if we can't afford it or if we've
                // already got enough of them
                continue;
            }

            // Start building new robot
            let mut new_balance = sub(&balance, cost);
            let mut new_robots = robots;
            new_robots[i] += 1;

            // Meanwhile collect ore
            new_balance = add(&new_balance, &robots);

            // Continue down the branch
            best = best.max(branch_and_bound(
                State {
                    balance: new_balance,
                    robots: new_robots,
                    countdown: countdown - 1,
                },
                blueprint,
                best,
            ));
        }

        // ... or don't build a new robot
        countdown -= 1;
        balance = add(&balance, &robots);
    }

    best
}

/// Returns the highest possible score for the current branch
fn upper_bound(state: State, blueprint: &Blueprint) -> u8 {
    let State {
        mut balance,
        mut robots,
        countdown,
    } = state;

    // Ignore ore, always build clay bots for free, build a geode bot if we can
    // afford it, build an obsidian bot if we can afford it.
    for _ in 0..countdown {
        let mut build = [0, 1, 0, 0];

        if blueprint.costs[2][1] <= balance[1] {
            balance[1] -= blueprint.costs[2][1];
            build[2] = 1;
        }

        if blueprint.costs[3][2] <= balance[2] {
            balance[2] -= blueprint.costs[3][2];
            build[3] = 1;
        }

        balance = add(&balance, &robots);
        robots = add(&robots, &build);
    }

    balance[3]
}
