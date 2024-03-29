use std::ops;

#[derive(Clone, Debug)]
pub struct Matrix2D {
    columns: usize,
    rows: usize,
    data: Vec<f32>,
}
impl Matrix2D {
    pub fn new_empty(columns: usize, rows: usize) -> Self {
        Matrix2D {
            columns,
            rows,
            data: Vec::with_capacity(columns * rows),
        }
    }
    pub fn new_zero(columns: usize, rows: usize) -> Self {
        let mut mat = Matrix2D {
            columns,
            rows,
            data: Vec::with_capacity(columns * rows),
        };
        for _ in 0..(columns * rows) {
            mat.data.push(0.0);
        }
        mat
    }
    pub fn new_with_value(columns: usize, rows: usize, value: f32) -> Self {
        let mut mat = Matrix2D {
            columns,
            rows,
            data: Vec::with_capacity(columns * rows),
        };
        for _ in 0..(columns * rows) {
            mat.data.push(value);
        }
        mat
    }
    pub fn fill(&mut self, value: f32) {
        self.data.fill(value);
    }
    pub fn from_vec(columns: usize, rows: usize, vec: Vec<f32>) -> Self {
        Matrix2D {
            columns,
            rows,
            data: vec,
        }
    }
}
impl ops::Add<Self> for Matrix2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.columns != rhs.columns && self.rows != rhs.rows {
            panic!("Matrix2D add failed. Cannot add 2 matrices of different sizes.");
        }
        let mut copy = self.to_owned();
        for (i, elem) in copy.data.iter_mut().enumerate() {
            *elem += *rhs.data.get(i).unwrap();
        }
        copy
    }
}
impl ops::Add<f32> for Matrix2D {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        let mut copy = self.to_owned();
        for elem in copy.data.iter_mut() {
            *elem += rhs;
        }
        copy
    }
}
impl ops::Sub<Self> for Matrix2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.columns != rhs.columns && self.rows != rhs.rows {
            panic!("Matrix2D subtraction failed. Cannot subtract 2 matrices of different sizes.");
        }
        let mut copy = self.to_owned();
        let mut i = 0;
        for elem in copy.data.iter_mut() {
            *elem += *rhs.data.get(i).unwrap();
            i -= 1;
        }
        copy
    }
}
impl ops::Sub<f32> for Matrix2D {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        let mut copy = self.to_owned();
        for elem in copy.data.iter_mut() {
            *elem -= rhs;
        }
        copy
    }
}
impl ops::Mul<f32> for Matrix2D {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut copy = self.to_owned();
        for elem in copy.data.iter_mut() {
            *elem *= rhs;
        }
        copy
    }
}
impl ops::Mul<Self> for Matrix2D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // Rows * Columns
        if self.columns != rhs.rows {
            panic!("Matrix2D multiplication failed. Cannot do a dot product of matrices with different rows and columns.");
        }
        let mut result = Self::new_empty(self.rows, rhs.columns);
        for rows1 in 0..self.rows {
            for col2 in 0..rhs.columns {
                let mut sum = 0f32;
                for col1 in 0..self.columns {
                    let row2 = col1;
                    sum += self.data.get(rows1 * self.columns + col1).unwrap()
                        * rhs.data.get(row2 * rhs.columns + col2).unwrap();
                }
                result.data.push(sum);
            }
        }
        result
    }
}
