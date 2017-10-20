use std::ops::{Add, Mul};
use matrix::{Matrix, MatrixResult};

#[derive(PartialEq, Clone, Debug)]
pub enum Command<T> {
    AddRow { coeff: T, src: usize, dest: usize },
    ScaleRow { coeff: T, row: usize },
    SwapRow { row1: usize, row2: usize },
    EditCell { row: usize, col: usize, value: T }
}

impl <T> Command<T> {
    pub fn apply(&self, matrix: &Matrix<T>) -> MatrixResult<Matrix<T>>
        where T: Clone + Add<T, Output = T> + Mul<T, Output = T> {
        let mut copy = matrix.clone();
        let _: () = match self {
            &Command::AddRow { ref coeff, src, dest } => {
                let src_row = matrix.get_row(src);
                let dest_row = matrix.get_row(dest);
                let new_dest: Vec<T> = src_row.iter()
                    .zip(dest_row)
                    .map(|(s, d)| coeff.clone() * s.clone() + d)
                    .collect();
                copy.set_row(dest, &new_dest)?;
            },
            &Command::ScaleRow { ref coeff, row } => {
                copy.row_foreach(row, |x| coeff.clone() * x.clone())?;
            },
            &Command::SwapRow { row1, row2 } => {
                let r1 = matrix.get_row(row1);
                let r2 = matrix.get_row(row2);
                copy.set_row(row1, &r2)?;
                copy.set_row(row2, &r1)?;
            },
            &Command::EditCell { row, col, ref value } => {
                copy.set(row, col, value.clone())?;
            }
        };
        Ok(copy)
    }
}
