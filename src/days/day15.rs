//! A process of elimination
//! 
//! A circle in taxi-cab space takes the shape of a diamond. Since each sensor
//! scans a circle of fixed radius, each one sweeps out a diamond on the grid.
//! 
//! If only a single space is left unscanned, then each adjacent space must lie
//! on the edge of a sensor diamond. Each edge of a sensor diamond lies along
//! a line with equation y=b+x or y=b-x. Rotating our coordinate system by
//! 45 degrees, x'=x+y and y'=x-y, each diamond becomes a square with edges 
//! parallel to the axes. The point at which two edges of different squares
//! intersect will therefore simply by (a, b) where the lines have
//! intercepts (a, 0) and (0, b) respectively. 
//! 
//! Since the number of diamonds is pretty small, the number of intersections
//! will be pretty small, so we can just enumerate all the intersections and
//! then loop over them to find the empty square. Specifically, we loop over all 
//! *top-edge* intersections, noting that the empty square must be just above 
//! such a point.

use std::{fs::read_to_string, collections::HashSet};

use crate::parse;

const HEIGHT: i32 = 2_000_000;
const DISTRESS_WINDOW: i32 = 4_000_000;

// Data class ------------------------------------------------------------------
type Point = (i32, i32);

#[derive(Debug)]
struct Diamond {
    centre: Point,
    radius: i32 // Diamonds are actually circles in taxi-cab space
}

impl Diamond {
    fn new(centre: Point, edge: Point) -> Self {
        let radius = centre.0.abs_diff(edge.0) + centre.1.abs_diff(edge.1);
        Diamond {
            centre,
            radius: radius as i32
        }
    }

    /// Return the end-points of the horizontal band at a given y-coordinate
    fn band(&self, y: i32) -> Option<(i32, i32)> {
        let vdist = y.abs_diff(self.centre.1);
        let budget = self.radius - vdist as i32;
        
        if budget >= 0 {
            Some((self.centre.0 - budget, self.centre.0 + budget))
        } else {
            None
        }
    }

    /// Returns true if the point is within the diamond
    fn within(&self, x: Point) -> bool {
        self.centre.0.abs_diff(x.0) + self.centre.1.abs_diff(x.1) <= (self.radius as u32)
    }

    /// Returns the location of the top of the diamond
    fn top(&self) -> Point {
        (self.centre.0, self.centre.1 + self.radius)
    }
}

// Parsing ---------------------------------------------------------------------

/// Returns a tuple containing a vector of sensor diamonds and a vector of
/// the corresponding beacons
fn parse() -> (Vec<Diamond>, Vec<Point>) {
    let input = read_to_string("data/day15.txt").unwrap();
    let reports = parse::parse_i32(&input);

    let mut diamonds = Vec::new();
    let mut beacons = Vec::new();

    for report in reports.chunks(4) {
        if let &[sx, sy, bx, by] = report {
            diamonds.push(Diamond::new((sx, sy), (bx, by)));
            beacons.push((bx, by));
        }
    }

    (diamonds, beacons)
}

// Solution --------------------------------------------------------------------

/// Returns the number of eliminated positions on the row at `HEIGHT`
pub fn part1() -> i32 {
    let (diamonds, beacons) = parse();
    
    let occupied: HashSet<i32> = beacons
        .iter()
        .filter(|(_, y)| *y == HEIGHT)
        .map(|(x, _)| *x)
        .collect();

    let mut eliminated: Vec<(i32, i32)> = diamonds
        .iter()
        .filter_map(|o| o.band(HEIGHT))
        .collect();

    eliminated.sort_by_key(|(x, _)| *x);

    // Collect up the ranges
    let mut solution = 0;
    let mut right = i32::MIN;
    for (l, r) in eliminated {
        if (l < right) & (r > right) {
            solution += r - right;
            right = r;
        } else if r > right {
            solution += r - l;
            right = r;
        }

    }

    solution + 1 - (occupied.len() as i32)
}


/// Returns the tuning frequency of the distress beacon
pub fn part2() -> u64 {
    let (diamonds, _) = parse();
    let tops: Vec<Point> = diamonds
        .iter()
        .map(|d| rotate(d.top()))
        .collect();

    let mut intersections = Vec::new();
    for (trx, _) in &tops {
        for (_, tly) in &tops {
            // Locations where top-left and top-right edges intersect,
            let (x, y) = unrotate((*trx, *tly));
            if (0..=DISTRESS_WINDOW).contains(&x) & (1..=(DISTRESS_WINDOW+1)).contains(&y) {
                // Empty space will be just above the intersection point
                intersections.push((x, y + 1));  
            }
        }
    }

    // The number of intersections will be pretty small, so we can just
    // iterate over them to find the one space outside of each diamond.
    intersections.retain(|&pt| diamonds.iter().all(|d| !d.within(pt)));

    (intersections[0].0 as u64) * (DISTRESS_WINDOW as u64) + (intersections[0].1 as u64)
}

fn rotate((x, y): Point) -> Point {
    (x+y, x-y)
}

fn unrotate((x, y): Point) -> Point {
    ((x+y)/2, (x-y)/2)
}