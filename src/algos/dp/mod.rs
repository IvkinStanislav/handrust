use std::ops::{Index, IndexMut, Range};

pub mod backpack;
pub mod calculator;
pub mod editing_distance;
pub mod exchange;
pub mod lcs;
pub mod place_brackets;
pub mod souvenirs;

#[derive(Debug)]
pub struct Matrix<T> {
    data: Vec<T>,
    d2_size: usize,
}

impl<T: Clone + Default> Matrix<T> {
    pub fn new(d1: usize, d2: usize) -> Self {
        Self::new_with_init_value(Default::default(), d1, d2)
    }
}

impl<T: Clone> Matrix<T> {
    pub fn new_with_init_value(init_value: T, d1: usize, d2: usize) -> Self {
        Self {
            data: vec![init_value; d1 * d2],
            d2_size: d2,
        }
    }
}

impl<T> Matrix<T> {
    fn d1_range(&self, i1: usize) -> Range<usize> {
        i1 * self.d2_size..(i1 + 1) * self.d2_size
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (i1, i2): (usize, usize)) -> &Self::Output {
        let d1_range = self.d1_range(i1);
        &self.data[d1_range][i2]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (i1, i2): (usize, usize)) -> &mut Self::Output {
        let d1_range = self.d1_range(i1);
        &mut self.data[d1_range][i2]
    }
}

#[derive(Debug)]
pub struct Cube<T> {
    data: Vec<T>,
    d2_size: usize,
    d3_size: usize,
}

impl<T: Clone + Default> Cube<T> {
    pub fn new(d1: usize, d2: usize, d3: usize) -> Self {
        Self::new_with_init_value(Default::default(), d1, d2, d3)
    }
}

impl<T: Clone> Cube<T> {
    pub fn new_with_init_value(init_value: T, d1: usize, d2: usize, d3: usize) -> Self {
        Self {
            data: vec![init_value; d1 * d2 * d3],
            d2_size: d2 * d3,
            d3_size: d3,
        }
    }
}

impl<T> Cube<T> {
    fn d1_range(&self, i: usize) -> Range<usize> {
        i * self.d2_size..(i + 1) * self.d2_size
    }

    fn d2_range(&self, i: usize) -> Range<usize> {
        i * self.d3_size..(i + 1) * self.d3_size
    }
}

impl<T> Index<(usize, usize, usize)> for Cube<T> {
    type Output = T;

    fn index(&self, (i1, i2, i3): (usize, usize, usize)) -> &Self::Output {
        let d1_range = self.d1_range(i1);
        let d2_range = self.d2_range(i2);
        &self.data[d1_range][d2_range][i3]
    }
}

impl<T> IndexMut<(usize, usize, usize)> for Cube<T> {
    fn index_mut(&mut self, (i1, i2, i3): (usize, usize, usize)) -> &mut Self::Output {
        let d1_range = self.d1_range(i1);
        let d2_range = self.d2_range(i2);
        &mut self.data[d1_range][d2_range][i3]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn matrix_test() {
        let mut matrix: Matrix<usize> = Matrix::new(3, 3);
        matrix[(0, 0)] = 1;
        matrix[(0, 1)] = 2;
        matrix[(0, 2)] = 3;

        matrix[(1, 0)] = 4;
        matrix[(1, 1)] = 5;
        matrix[(1, 2)] = 6;

        matrix[(2, 0)] = 7;
        matrix[(2, 1)] = 8;
        matrix[(2, 2)] = 9;

        assert_eq!(matrix.data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
    }

    #[test]
    fn cube_test() {
        let mut cube: Cube<usize> = Cube::new(3, 3, 3);
        cube[(0, 0, 0)] = 1;
        cube[(0, 0, 1)] = 2;
        cube[(0, 0, 2)] = 3;

        cube[(0, 1, 0)] = 4;
        cube[(0, 1, 1)] = 5;
        cube[(0, 1, 2)] = 6;

        cube[(0, 2, 0)] = 7;
        cube[(0, 2, 1)] = 8;
        cube[(0, 2, 2)] = 9;

        cube[(1, 0, 0)] = 10;
        cube[(1, 0, 1)] = 11;
        cube[(1, 0, 2)] = 12;

        cube[(1, 1, 0)] = 13;
        cube[(1, 1, 1)] = 14;
        cube[(1, 1, 2)] = 15;

        cube[(1, 2, 0)] = 16;
        cube[(1, 2, 1)] = 17;
        cube[(1, 2, 2)] = 18;

        cube[(2, 0, 0)] = 19;
        cube[(2, 0, 1)] = 20;
        cube[(2, 0, 2)] = 21;

        cube[(2, 1, 0)] = 22;
        cube[(2, 1, 1)] = 23;
        cube[(2, 1, 2)] = 24;

        cube[(2, 2, 0)] = 25;
        cube[(2, 2, 1)] = 26;
        cube[(2, 2, 2)] = 27;

        assert_eq!(
            cube.data,
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27
            ]
        )
    }
}
