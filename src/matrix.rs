// use crate::utils::cmp_f32;
use std::error::Error;
#[derive(PartialEq, Debug, Clone)]
pub struct Matrix<T: Copy + PartialEq> {
    data: Vec<T>,
    n_rows: usize,
    n_cols: usize,
}
impl<T: Copy + PartialEq> Matrix<T> {
    pub fn new(n_rows: usize, n_cols: usize, data: Vec<T>) -> Result<Matrix<T>, Box<dyn Error>> {
        if n_rows * n_cols != data.len() {
            return Err(Box::from("not enough or to much data"));
        }
        Ok(Matrix {
            data,
            n_rows,
            n_cols,
        })
    }
    //get cell by row and column and return error if out of bounds
    pub fn get(&self, row: usize, col: usize) -> Result<T, Box<dyn Error>> {
        if row >= self.n_rows || col >= self.n_cols {
            return Err(Box::from("out of bounds"));
        }
        Ok(self.data[row * self.n_cols + col])
    }

    //set cell by row and column and return error if out of bounds
    pub fn set(&mut self, row: usize, col: usize, value: T) -> Result<T, Box<dyn Error>> {
        if row >= self.n_rows || col >= self.n_cols {
            return Err(Box::from("out of bounds"));
        }
        self.data[row * self.n_cols + col] = value;
        Ok(value)
    }
}
