use std::ops::{Add, Sub, Mul, AddAssign, SubAssign, MulAssign};
use std::fmt;

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