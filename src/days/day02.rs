
#[derive(PartialEq, Eq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

/// Convert character to shape
fn parse_shape(x: &u8) -> Result<Shape, String> {
    match x {
        b'A' | b'X' => Ok(Shape::Rock),
        b'B' | b'Y' => Ok(Shape::Paper),
        b'C' | b'Z' => Ok(Shape::Scissors),
        _ => Err("Unrecognised shape".to_string())
    }
}

/// Read shapes from input, return a vector of tuples containing the shapes 
/// used in each rock-paper-scissors game.
/// 
/// Panics if input is not in expected format
fn read_shapes() -> Vec<(Shape, Shape)> {
    return include_bytes!("../../data/day02.txt")
        .split(|b| *b == b'\n')
        .map(|l| (parse_shape(&l[0]).unwrap(), parse_shape(&l[2]).unwrap()))
        .collect()
}

/// Return the score for a given shape
fn score_shape(shape: &Shape) -> u8 {
    return match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3
    }
}

/// Return my score depending on whether I win, draw or lose
fn score_match(me: &Shape, elf: &Shape) -> u8 {
    return match (me, elf) {
        (x, y) if x == y => 3,
        (Shape::Rock, Shape::Scissors) => 6,
        (Shape::Paper, Shape::Rock) => 6,
        (Shape::Scissors, Shape::Paper) => 6,
        _ => 0
    }
}

/// Return a new shape based on my and the elf's choices
/// 
/// I will swap a Rock to the losing shape, a Paper to a drawing shape,
/// and a Scissors to a winning shape.
fn swap_shape(me: &Shape, elf: &Shape) -> Shape {
    match me {
        Shape::Rock => {
            match elf {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper
            }
        },
        Shape::Scissors => {
            match elf {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock
            }
        }
        Shape::Paper => elf.clone()
    }
}

pub fn part1() -> u32 {
    let mut solution: u32 = 0;

    for (elf, me) in read_shapes() {
        solution += (score_shape(&me) + score_match(&me, &elf)) as u32
    }

    solution
}

pub fn part2() -> u32 {
    let mut solution: u32 = 0;
    let mut my_shape: Shape;

    for (elf, me) in read_shapes() {
        my_shape = swap_shape(&me, &elf);
        solution += (score_shape(&my_shape) + score_match(&my_shape, &elf)) as u32
    }
    
    solution
}
