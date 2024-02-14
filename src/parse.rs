use std::str;

/// Returns a vector containing all the unsigned integers in the string
pub fn parse_u32(s: &str) -> Vec<u32> {
    s.as_bytes()
        .split(|b| !b.is_ascii_digit())
        .filter(|x| !x.is_empty())
        .map(|x| str::from_utf8(x).unwrap().parse::<u32>().unwrap())
        .collect()
}

/// Returns a vector containing all the unsigned integers in the string
pub fn parse_i32(s: &str) -> Vec<i32> {
    s.as_bytes()
        .split(|b| !(*b == b'-' || b.is_ascii_digit()))
        .filter(|x| !(x.is_empty() || *x == [b'-']))
        .map(|x| str::from_utf8(x).unwrap().parse::<i32>().unwrap())
        .collect()
}