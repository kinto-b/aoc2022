//! Counting visible trees
//! 
//! Grid is 99x99 with \n\r terminators, so X[i, j] is at x[i*101, j].

const WIDTH: usize = 99;

/// Returns a 99x99 grid stored as a 9801 length vector
fn parse() -> Vec<u8> {
    include_bytes!("../../data/day08.txt")
        .chunks(WIDTH+2) // row plus '\n\r'
        .flat_map(|l| l.strip_suffix(b"\r\n").unwrap_or(l))
        .copied()
        .collect()
}

/// Count the number of trees visible from the outside
pub fn part1() -> usize {
    let x = parse();

    let mut visibility = vec![false; x.len()];
    let mut pos: usize;
    for i in 0..WIDTH {
        let [mut hl, mut hr, mut ht, mut hb] = [0; 4];

        for j in 0..WIDTH {
            // Moving L-R
            pos = i*WIDTH + j;
            if x[pos] > hl {
                hl = x[pos];
                visibility[pos] = true;
            }

            // Moving R-L
            pos = i*WIDTH + WIDTH-j-1;
            if x[pos] > hr {
                hr = x[pos];
                visibility[pos] = true;
            }

            // Moving T-B
            pos = j*WIDTH + i;
            if x[pos] > ht {
                ht = x[pos];
                visibility[pos] = true;
            }

            // Moving B-T
            pos = (WIDTH-j-1)*WIDTH + i;
            if x[pos] > hb {
                hb = x[pos];
                visibility[pos] = true;
            }
        }
    }

    visibility.iter().filter(|b| **b).count()
}

/// Returns the maximum 'scenic score' from among all the trees
pub fn part2() -> u32 {
    let x = parse();

    let mut max_score = 0;
    for i in 1..(WIDTH-1) { // Don't need to consider exterior trees
        for j in 1..(WIDTH-1) {
            let score = scenic_score(i, j, &x);
            if score > max_score { 
                max_score = score;
             };
        }
    }

    max_score
}

/// Returns the scenic score at a given position
fn scenic_score(i: usize, j: usize, x: &Vec<u8>) -> u32 {
    let h = x[i*WIDTH + j];
    let [mut l, mut r, mut t, mut b] = [0; 4];
    
    // Moving L-R
    for k in 1..(WIDTH-j) {
        l += 1;
        if h <= x[i*WIDTH + j+k] { break; }
    }
    
    // Moving R-L
    for k in 1..(j+1) {
        r += 1;
        if h <= x[i*WIDTH + j-k] { break; }
    }
    
    // Moving T-B
    for k in 1..(WIDTH-i) {
        t += 1;
        if h <= x[(i+k)*WIDTH + j] { break; }
    }
    
    // Moving B-T
    for k in 1..(i+1) {
        b += 1;
        if h <= x[(i-k)*WIDTH + j] { break; }
    }

    l*r*t*b
}