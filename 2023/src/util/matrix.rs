// matrix.rs

use std::{
    fmt::{Debug, Display},
    iter,
};

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    elements: Vec<Vec<T>>,
    xlen: usize,
    ylen: usize,
}

impl<T> Matrix<T>
where
    T: Clone + Copy + Display,
{
    pub fn new(xlen: usize, ylen: usize, default: T) -> Self {
        let mut elements: Vec<Vec<T>> = Vec::new();
        for _ in 0..ylen {
            let row: Vec<T> = iter::repeat(default).take(xlen).collect_vec();
            elements.push(row);
        }
        Matrix {
            elements: elements,
            xlen: xlen,
            ylen: ylen,
        }
    }

    pub fn xlen(&self) -> usize {
        self.xlen
    }
    pub fn ylen(&self) -> usize {
        self.ylen
    }

    pub fn set(&mut self, pos: (usize, usize), val: &T) {
        // self.check_pos(pos);
        let old = self.elements[pos.1].get_mut(pos.0).unwrap();
        *old = *val;
    }
    pub fn get(&self, pos: (usize, usize)) -> &T {
        // self.check_pos(pos);
        let val = self.elements[pos.1].get(pos.0).unwrap();
        val
    }

    pub fn rotate_right(&self) -> Matrix<T> {
        let mut elements = Vec::new();
        for x in 0..self.xlen {
            let mut row = Vec::new();
            for y in (0..self.ylen).rev() {
                let val = self.get((x, y));
                row.push(*val);
            }
            elements.push(row)
        }

        let new_matrix = Matrix {
            xlen: self.ylen,
            ylen: self.xlen,
            elements: elements,
        };
        new_matrix
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for row in self.elements.iter() {
            let mut row_result = String::new();
            for ele in row {
                let out = format!("{}", &ele);
                row_result.push_str(&out);
            }
            row_result.push('\n');
            result.push_str(&row_result)
        }
        result
    }
}
