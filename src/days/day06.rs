
/// Return the index of the first character which is preceeded by `size` unique
/// characters (inclusive of final character).
fn locate_marker(size: usize) -> usize {
    let buffer = include_bytes!("../../data/day06.txt");
    let mut window: Vec<u8> = Vec::new();

    for (i, x) in buffer.iter().enumerate() {
        if let Some(i) = window.iter().position(|w| w == x) {
            window.drain(..(i+1));
        }
        window.push(*x);

        if window.len() == size {
            return i + 1;
        }
    }

    unreachable!()
}

pub fn part1() -> usize {
    locate_marker(4)
}

pub fn part2() -> usize {
    locate_marker(14)
}