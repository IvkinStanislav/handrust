use std::ops::{Index, IndexMut, Range};

pub mod backpack;
pub mod calculator;
pub mod editing_distance;
pub mod exchange;
pub mod lcs;
pub mod place_brackets;
pub mod souvenirs;

pub struct Matrix<T> {
    data: Vec<T>,
    row_len: usize
}

impl<T: Clone> Matrix<T> {
    pub fn new_with_init_value(init_value: T, n: usize, m: usize) -> Self {
        Self {
            data: vec![init_value; n * m],
            row_len: m,
        }
    }
}

impl<T> Matrix<T> {
    fn get_row_range(&self, row_index: usize) -> Range<usize> {
        row_index * self.row_len .. (row_index + 1) * self.row_len
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let row_range = self.get_row_range(index);
        &self.data[row_range]
    }
}


impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let row_range = self.get_row_range(index);
        &mut self.data[row_range]
    }
}
