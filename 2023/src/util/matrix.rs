// matrix.rs

use core::fmt;
use std::{fmt::Debug, iter};

use itertools::Itertools;

#[derive(Debug)]
pub struct Matrix<T> {
    elements: Vec<Vec<T>>,
    xlen: usize,
    ylen: usize,
}

// impl<T> fmt::Debug for Matrix<T>
// where
//     T: Debug,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Matrix").field(name, value)
//         f.debug_tuple("Size")
//             .field(&self.xlen)
//             .field(&self.ylen)
//             .finish()

//         // f.debug_struct("Matrix")
//         //     .field("elements", &self.elements)
//         //     .field("xlen", &self.xlen)
//         //     .field("ylen", &self.ylen)
//         //     .finish()
//     }
// }

impl<T> Matrix<T>
where
    T: Clone + Copy,
{
    pub fn new(xlen: usize, ylen: usize, default: T) -> Self {
        let mut elements: Vec<Vec<T>> = Vec::new();
        for y in 0..ylen {
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

    fn check_pos(&self, pos: (usize, usize)) {
        if (pos.0 >= self.xlen || pos.1 >= self.ylen) {
            panic!("bad position")
        }
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

        let mut new_matrix = Matrix {
            xlen: self.ylen,
            ylen: self.xlen,
            elements: elements,
        };
        new_matrix
    }
}
