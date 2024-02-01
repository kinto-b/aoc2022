use std::{fs::read_to_string, vec};

type Instruction = (usize, usize, usize);
type Stack = Vec<char>;

// Parsing ---------------------------------------------------------------------

fn parse_stacks(x: &str) -> Vec<Stack> {
    let layers: Vec<Vec<char>> = x.lines()
        .map(|l| l.chars().collect())
        .rev()
        .collect();

    let nstacks = layers[0].len() / 4;
    let mut stacks: Vec<Stack> = vec![Vec::new(); nstacks]; 

    // First layer is stack indices, so skip it
    for layer in layers.iter().skip(1) {
        // First character is '[', so skip it
        // Thereafter, stacks are separated by '] ['
        for (i, c) in layer.iter().skip(1).step_by(4).enumerate() {
            if !c.is_whitespace() {
                stacks[i].push(*c);
            }
        }
    }

    stacks
}

fn parse_instructions(x: &str) -> Vec<Instruction> {
    x.lines()
        .map(_parse_instruction)
        .collect()
}

fn _parse_instruction(x: &str) -> Instruction {
    let x = x[5..] // Instructions begin with 'move '
        .replace("from", "")
        .replace("to", "");
    
    let mut m = x.split_whitespace()
        .map(|v| v.parse::<usize>().unwrap());

    (m.next().unwrap(), m.next().unwrap()-1, m.next().unwrap()-1)
}

fn parse() -> (Vec<Stack>, Vec<Instruction>) {
    let input = read_to_string("data/day05.txt").unwrap();
    let (stacks, instructions) = input
        .split_once("\n\r\n")
        .unwrap();

    (parse_stacks(stacks), parse_instructions(instructions))
}

// Logic -----------------------------------------------------------------------

pub fn part1() -> String {
    let (mut stacks, instructions) = parse();
    
    for (n, from, to) in instructions {
        for _ in 0..n {
            let v = stacks[from].pop().unwrap();
            stacks[to].push(v);
        }
    }

    stacks.iter().map(|s| s.last().unwrap()).collect()
}

pub fn part2() -> String {
    let (mut stacks, instructions) = parse();
    let mut collected: Stack = Vec::new(); // To satisfy the borrow checker
    
    for (n, from, to) in instructions {
        let left = stacks[from].len() - n;
        collected.extend(stacks[from].drain(left..));
        stacks[to].append(&mut collected);
    }

    stacks.iter().map(|s| s.last().unwrap()).collect()
}