use std::fmt;
use std::ops::{Add, IndexMut, Sub};

#[derive(PartialEq, Debug, Clone)]
/// Matrix that contains the specified data.
pub struct Matrix<T> {
    data: Vec<T>,
    pub rows: usize,
    pub cols: usize
}

impl <T> Matrix<T> {

    /// Creates a matrix with the given size that contains the given data.
    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> Matrix<T> {
        assert!(data.len() == rows * cols);
        Matrix {
            data: data,
            rows: rows,
            cols: cols
        }
    }

    /// Creates a new matrix with the given size that contains the given value.
    pub fn filled(rows: usize, cols: usize, default: T) -> Matrix<T>
        where T: Clone {
        Matrix::new(rows, cols, vec![default; rows * cols])
    }

    /// Gets the value in the specified row and column, if the row and column
    /// is contained in the matrix.
    pub fn get(&self, row: usize, col: usize) -> Option<T> where T: Clone {
        if row > self.rows || col > self.cols || row < 1 || col < 1 {
            None
        } else {
            Some(self.data[(row - 1) * self.cols + (col - 1)].clone())
        }
    }

    pub fn get_row<'a>(&self, row: usize) -> Vec<T> where T: Clone {
        let start = (row - 1) * self.cols;
        self.data[start..(start + self.cols)].to_owned()
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) -> () {
        assert!(row <= self.rows && col <= self.cols && row > 0 && col > 0);
        self.data[(row - 1) * self.cols + (col - 1)] = value;
    }

    pub fn set_row(&mut self, row: usize, values: &[T]) -> () where T: Clone {
        assert!(row <= self.rows && values.len() <= self.cols && row > 0);
        let start = (row - 1) * self.cols;
        let slice = self.data.index_mut(start..(start + self.cols));
        for i in 0..slice.len() {
            slice[i] = values[i].clone();
        }
    }

    /// Changes each element of the row in-place with the given function
    pub fn row_foreach<F>(&mut self, row: usize, cb: F) -> ()
        where F: Fn(&T) -> T
    {
        assert!(row <= self.rows && row > 0);
        let start = (row - 1) * self.cols;
        let slice = self.data.index_mut(start..(start + self.cols));
        for i in 0..slice.len() {
            slice[i] = cb(&slice[i]);
        }
    }

    pub fn rows(&self) -> Vec<Vec<T>> where T: Clone {
        self.data
            .chunks(self.cols)
            .map(|sl| sl.to_vec())
            .collect()
    }
}

impl <'a, T: Add<T, Output = T> + Clone> Add for &'a Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, rhs: &Matrix<T>) -> Self::Output {
        assert!(self.rows == rhs.rows && self.cols == rhs.cols);
        let new_data = self.data.iter().zip(&rhs.data).map(|(a, b)| a.clone() + b.clone()).collect();
        Matrix::new(self.rows, self.cols, new_data)
    }
}

impl <'a, T: Sub<T, Output = T> + Clone> Sub for &'a Matrix<T> {
    type Output = Matrix<T>;

    fn sub(self, rhs: &Matrix<T>) -> Self::Output {
        assert!(self.rows == rhs.rows && self.cols == rhs.cols);
        let new_data = self.data.iter().zip(&rhs.data).map(|(a, b)| a.clone() - b.clone()).collect();
        Matrix::new(self.rows, self.cols, new_data)
    }
}

impl <T: Clone + fmt::Display> fmt::Display for Matrix<T> {
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
