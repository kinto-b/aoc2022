
fn elf_totals() -> Vec<i32> {
    include_str!("../../data/day01.txt").
        split("\r\n\r\n")
        .map(|v| v.lines().map(|i| i.parse::<i32>().unwrap()).sum())
        .collect()
}


pub fn part1() -> i32 {
    *elf_totals().iter().max().unwrap()
}

pub fn part2() -> i32 {
    let mut totals = elf_totals();
    totals.sort();

    totals.iter().rev().take(3).sum()
}