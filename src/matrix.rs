// use crate::utils::cmp_f32;
use std::error::Error;

use crate::tuple::Tuple;
#[derive(PartialEq, Debug, Clone)]
pub struct Matrix<T: Copy + PartialEq> {
    data: Vec<T>,
    pub n_rows: usize,
    pub n_cols: usize,
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
    pub fn _get(&self, row: usize, col: usize) -> Result<T, Box<dyn Error>> {
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

impl<T: Copy + PartialEq> std::ops::Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.n_cols..(index + 1) * self.n_cols]
    }
}

impl<T: Copy + PartialEq + Default + std::ops::Mul<Output = T> + std::ops::Add<Output = T>>
    std::ops::Mul for Matrix<T>
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        if (self.n_cols, self.n_rows, rhs.n_cols, rhs.n_rows) != (4, 4, 4, 4) {
            panic!("matrices must be 4x4");
        }
        let data = vec![T::default(); 16];
        let mut result = Matrix::new(4, 4, data).unwrap();
        for row in 0..4 {
            for col in 0..4 {
                result
                    .set(
                        row,
                        col,
                        self[row][0] * rhs[0][col]
                            + self[row][1] * rhs[1][col]
                            + self[row][2] * rhs[2][col]
                            + self[row][3] * rhs[3][col],
                    )
                    .unwrap();
            }
        }
        result
    }
}

impl<
        T: Copy + PartialEq + Default + std::ops::Mul<f32, Output = T> + std::ops::Add<Output = T>,
    > std::ops::Mul<Tuple> for Matrix<T>
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Tuple) -> Self::Output {
        if (self.n_cols, self.n_rows) != (4, 4) {
            panic!("matrices must be 4x4");
        }
        let data = vec![T::default(); 16];
        let mut result = Matrix::new(4, 4, data).unwrap();
        for row in 0..4 {
            for col in 0..4 {
                result
                    .set(
                        row,
                        col,
                        self[row][0] * rhs[0]
                            + self[row][1] * rhs[1]
                            + self[row][2] * rhs[2]
                            + self[row][3] * rhs[3],
                    )
                    .unwrap();
            }
        }
        result
    }
}
