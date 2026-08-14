use std::ops::{Add, Sub, Mul, AddAssign, SubAssign, MulAssign};
use std::fmt;
use crate::Vector;

#[derive(Clone, Debug)]
pub struct Matrix<K> {
    pub data: Vec<Vec<K>>,
}

impl<K> Matrix <K>
{
    pub fn shape(&self) -> (usize, usize){
        let rows = self.data.len();
        let cols = if rows == 0 {0} else {self.data[0].len()};
        (rows, cols)
    }

    pub fn new(data: Vec<Vec<K>>)  -> Self{
        Self { data }
    }

   pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K>
    where
        K: Copy + Mul<Output = K> + AddAssign + Default, 
    {
        let cols = if self.data.is_empty() { 0 } else { self.data[0].len() };
        if cols != vec.data.len() {
            panic!("Matrix columns must match vector length");
        }
        let mut result = Vec::new();
        for row in &self.data {
            let mut sum = K::default();
            for (&val, &v_val) in row.iter().zip(vec.data.iter()) {
                sum += val * v_val;
            }
            result.push(sum);
        }
        Vector::new(result)
    }

    pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K>
    where
        K: Copy + Mul<Output = K> + AddAssign + Default,
    {
        let (rows_a, cols_a) = self.shape();
        let (rows_b, cols_b) = mat.shape();
        if cols_a != rows_b {
            panic!("Matrix A columns must match Matrix B rows");
        }
        let mut result = Vec::with_capacity(rows_a);

        for i in 0..rows_a {
            let mut new_row = Vec::with_capacity(cols_b);
            for j in 0..cols_b {
                let mut sum = K::default();
                for k in 0..cols_a {
                    sum += self.data[i][k] * mat.data[k][j];
                }
                new_row.push(sum);
            }
            result.push(new_row);
        }
        Matrix::new(result)
    }

    pub fn trace(&self) -> K
    where
        K: Copy + AddAssign + Default,
    {
        let (rows, cols) = self.shape();
        if rows != cols {
            panic!("Matrix must be square");
        }
        let mut sum = K::default();
        for i in 0..rows {
            sum += self.data[i][i];
        }
        sum
    }

    pub fn transpose(&self) -> Matrix<K>
    where
        K: Copy,
    {
        let (rows, cols) = self.shape();
        if rows == 0 || cols == 0 {
            return Matrix::new(Vec::new());
        }
        let mut result = Vec::with_capacity(cols);
        for j in 0..cols {
            let mut new_row = Vec::with_capacity(rows);
            for i in 0..rows {
                new_row.push(self.data[i][j]);
            }
            result.push(new_row);
        }
        Matrix::new(result)
    }

}

impl<K: AddAssign + Copy> Matrix<K> {
    pub fn add (&mut self, v: &Matrix<K>){
        if self.shape() != v.shape(){
            panic!("The matrices must have the same shape");
        }
        for i in 0..v.data.len() {
            for j in 0..self.data[i].len(){
                self.data[i][j] += v.data[i][j];
            }
        }
    }
}

impl<K: SubAssign + Copy> Matrix<K> {
    pub fn sub (&mut self, v: &Matrix<K>){
        if self.shape() != v.shape(){
            panic!("The matrices must have the same shape");
        }
        for i in 0..v.data.len() {
            for j in 0..self.data[i].len(){
                self.data[i][j] -= v.data[i][j];
            }
        }
    }
}

impl<K: MulAssign + Copy> Matrix<K> {
    pub fn scl(&mut self, a: K){
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len(){
                self.data[i][j] *= a;
            }
        }
    }
}

impl<K> Add for Matrix<K>
where
    K: AddAssign + Copy,
{
    type Output = Self;
    fn add(mut self, v: Self) -> Self {
        Matrix::add(&mut self, &v);
        self
    }
}

impl<K> Sub for Matrix<K>
where
    K: SubAssign + Copy,
{
    type Output = Self;
    fn sub(mut self, v: Self) -> Self {
        Matrix::sub(&mut self, &v);
        self
    }
}

impl<K> Mul<K> for Matrix<K>
where
    K: MulAssign + Copy,
{
    type Output = Self;
    fn mul(mut self, a: K) -> Self {
        self.scl(a);
        self
    }
}

impl<K, const R: usize, const C: usize> From<[[K; C]; R]> for Matrix<K> {
    fn from(data: [[K; C]; R]) -> Self {
        let mut vec_data = Vec::with_capacity(R);
        for row in data {
            vec_data.push(Vec::from(row));
        }
        Self::new(vec_data)
    }
}

impl<K: fmt::Display> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.data.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "[")?;
            for (j, elem) in row.iter().enumerate() {
                if j > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", elem)?;
            }
            write!(f, "]")?;
        }
        Ok(())
    }
}