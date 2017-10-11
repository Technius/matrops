use std::fmt;
use std::ops::{Add, Index, IndexMut, Sub};

#[derive(PartialEq, Debug, Clone)]
pub struct Matrix {
    data: Vec<i32>,
    pub rows: usize,
    pub cols: usize
}

impl Matrix {

    /// Creates a matrix with the given size that contains the given data.
    pub fn new(rows: usize, cols: usize, data: Vec<i32>) -> Matrix {
        assert!(data.len() == rows * cols);
        Matrix {
            data: data,
            rows: rows,
            cols: cols
        }
    }

    /// Creates a new matrix with the given size that contains only zeroes.
    pub fn zero(rows: usize, cols: usize) -> Matrix {
        Matrix::new(rows, cols, vec![0; rows * cols])
    }

    /// Gets the value in the specified row and column, if the row and column
    /// is contained in the matrix.
    pub fn get(&self, row: usize, col: usize) -> Option<i32> {
        if row > self.rows || col > self.cols || row < 1 || col < 1 {
            None
        } else {
            Some(self.data[(row - 1) * self.cols + (col - 1)])
        }
    }

    // pub fn get_row<'a>(&self, row: usize) -> &'a [i32] {
    //     let start = (row - 1) * self.cols;
    //     self.data.index([start..(start + self.cols)])
    // }

    pub fn set(&mut self, row: usize, col: usize, value: i32) -> () {
        assert!(row <= self.rows && col <= self.cols && row > 0 && col > 0);
        self.data[(row - 1) * self.cols + (col - 1)] = value;
    }

    pub fn set_row(&mut self, row: usize, values: &[i32]) -> () {
        assert!(row <= self.rows && values.len() <= self.cols && row > 0);
        let start = (row - 1) * self.cols;
        let slice = self.data.index_mut(start..(start + self.cols));
        for i in 0..slice.len() {
            println!("{} {}", slice[i], values[i]);
            slice[i] = values[i];
        }
    }

    /// Changes each element of the row in-place with the given function
    pub fn row_foreach<F>(&mut self, row: usize, cb: F) -> ()
        where F: Fn(i32) -> i32
    {
        assert!(row <= self.rows && row > 0);
        let start = (row - 1) * self.cols;
        let slice = self.data.index_mut(start..(start + self.cols));
        for i in 0..slice.len() {
            slice[i] = cb(slice[i]);
        }
    }

    pub fn rows(&self) -> Vec<Vec<i32>> {
        self.data
            .chunks(self.cols)
            .map(|sl| sl.to_vec())
            .collect()
    }
}

impl <'a> Add for &'a Matrix {
    type Output = Matrix;

    fn add(self, rhs: &Matrix) -> Self::Output {
        assert!(self.rows == rhs.rows && self.cols == rhs.cols);
        let new_data = self.data.iter().zip(&rhs.data).map(|(a, b)| a + b).collect();
        Matrix::new(self.rows, self.cols, new_data)
    }
}

impl <'a> Sub for &'a Matrix {
    type Output = Matrix;

    fn sub(self, rhs: &Matrix) -> Self::Output {
        assert!(self.rows == rhs.rows && self.cols == rhs.cols);
        let new_data = self.data.iter().zip(&rhs.data).map(|(a, b)| a - b).collect();
        Matrix::new(self.rows, self.cols, new_data)
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for i in 1..(self.rows + 1) {
            for j in 1..(self.cols + 1) {
                buf.push_str(&format!("{}", self.get(i, j).unwrap()));
            }
            if i != self.rows {
                buf.push_str("\n");
            }
        }
        write!(f, "{}", buf)
    }
}
