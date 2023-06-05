// use crate::utils::cmp_f32;
use std::error::Error;

use crate::tuple::Tuple;
#[derive(PartialEq, Debug, Clone)]
pub struct Matrix {
    data: Vec<f32>,
    pub n_rows: usize,
    pub n_cols: usize,
}
impl Matrix {
    pub fn new(n_rows: usize, n_cols: usize, data: Vec<f32>) -> Result<Matrix, Box<dyn Error>> {
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
    pub fn _get(&self, row: usize, col: usize) -> Result<f32, Box<dyn Error>> {
        if row >= self.n_rows || col >= self.n_cols {
            return Err(Box::from("out of bounds"));
        }
        Ok(self.data[row * self.n_cols + col])
    }

    //set cell by row and column and return error if out of bounds
    pub fn set(&mut self, row: usize, col: usize, value: f32) -> Result<f32, Box<dyn Error>> {
        if row >= self.n_rows || col >= self.n_cols {
            return Err(Box::from("out of bounds"));
        }
        self.data[row * self.n_cols + col] = value;
        Ok(value)
    }

    /// generate an identity matrix
    /// by the form
    ///
    /// 1, 0, 0, 0,
    ///
    /// 0, 1, 0, 0,
    ///
    /// 0, 0, 1, 0,
    ///
    /// 0, 0, 0, 1,
    pub fn identity_matrix(n: usize) -> Matrix {
        let mut data = vec![f32::default(); n * n];
        for i in 0..n {
            data[i * n + i] = 1.0;
        }
        Matrix {
            data,
            n_rows: n,
            n_cols: n,
        }
    }

    pub fn transpose(self) -> Matrix {
        let mut data = vec![f32::default(); self.n_rows * self.n_cols];
        for row in 0..self.n_rows {
            for col in 0..self.n_cols {
                data[col * self.n_rows + row] = self.data[row * self.n_cols + col];
            }
        }
        Matrix {
            data,
            n_rows: self.n_cols,
            n_cols: self.n_rows,
        }
    }
}

impl std::ops::Index<usize> for Matrix {
    type Output = [f32];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.n_cols..(index + 1) * self.n_cols]
    }
}

impl std::ops::Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        if (self.n_cols, self.n_rows, rhs.n_cols, rhs.n_rows) != (4, 4, 4, 4) {
            panic!("matrices must be 4x4");
        }
        let data = vec![f32::default(); 16];
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

impl std::ops::Mul<Tuple> for &Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        if (self.n_cols, self.n_rows) != (4, 4) {
            panic!("matrices must be 4x4");
        }
        let mut result = Tuple::new(0.00, 0.0, 0.0, 0.0);
        for row in 0..4 {
            result[row] = self[row][0] * rhs[0]
                + self[row][1] * rhs[1]
                + self[row][2] * rhs[2]
                + self[row][3] * rhs[3]
        }
        result
    }
}
