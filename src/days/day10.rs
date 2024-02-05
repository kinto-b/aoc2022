use std::fs::read_to_string;

/// Returns a vector representing the register after each cycle
fn parse() -> Vec<i32> {
    let input = read_to_string("data/day10.txt").unwrap();
    let mut register: Vec<i32> = vec![1];

    for op in input.lines().map(|l| l.split_once(' ')) {
        let prev = *register.last().unwrap();
        register.push(prev);
        if let Some((_, x)) = op {
            register.push(x.parse::<i32>().unwrap() + prev)
        }
    }

    register.pop(); // Last record not needed.
    register
}

/// Returns the sum of the six 'signal strengths'
pub fn part1() -> i32 {
    parse()
        .iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(i, x)| (i as i32 + 1) * x)
        .sum()
}

/// Prints the CRT Screen
pub fn part2() -> String {
    let lit: Vec<char> = parse()
        .iter()
        .enumerate()
        .map(|(i, x)| pixel(i as i32, *x))
        .collect();

    println!("\nCRT Screen:");
    for line in lit.chunks(40) {
        println!("{}", line.iter().collect::<String>())
    }
    println!(" ");

    String::from("See above")
}

fn pixel(cursor: i32, sprite: i32) -> char {
    let cursor = cursor % 40;
    if (sprite - 1) <= cursor && cursor <= (sprite + 1) {
        '#'
    } else {
        '.'
    }
}
