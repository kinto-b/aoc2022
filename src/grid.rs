//! A two-dimensional grid


#[derive(Debug)]
pub struct Grid<T> {
    elements: Vec<T>,
    pub nrow: usize,
    pub ncol: usize
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let elements: Vec<u8> = input.as_bytes()
            .iter()
            .filter(|&i| *i > 31) // Remove whitespace
            .cloned()
            .collect();
        
        let nrow = input.lines().count();
        let ncol = elements.len() / nrow;

        Grid { elements, nrow, ncol }
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    pub fn new(elements: Vec<T>, nrow: usize) -> Self {
        let ncol = elements.len() / nrow;

        Grid { elements, nrow, ncol }
    }

    /// Returns the element at location (i, j)
    pub fn get(&self, i: usize, j: usize) -> T {
        self.elements[i*self.ncol + j]
    }

    /// Set the value of the element at location (i, j)
    pub fn set(&mut self, i: usize, j: usize, x: T) {
        self.elements[i*self.ncol + j] = x;
    }

    /// Returns the (first) index of a given element
    pub fn find(&self, x: T) -> Option<(usize, usize)> {
        self.elements.iter()
            .position(|&y| x==y)
            .map(|i| self._as_rowcol(i))
    }

    /// Returns the number of elements in the grid
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns indices of elements neighbouring (i, j)
    pub fn neighbours(&self, i: usize, j:usize) -> Vec<(usize, usize)> {
        let mut nb = Vec::with_capacity(4);
        
        if i > 0 { nb.push((i-1, j)) }
        if j > 0 { nb.push((i, j-1)) }
        if i < (self.nrow-1) { nb.push((i+1, j)) }
        if j < (self.ncol-1) { nb.push((i, j+1)) }

        nb  
    }

    /// Converts a position in the vector into a (row, col) location
    fn _as_rowcol(&self, idx: usize) -> (usize, usize) {
        (idx / self.ncol, idx % self.ncol)
    }
}
