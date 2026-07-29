use std::ops::{AddAssign, SubAssign, MulAssign};
use std::fmt;

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

impl <K: AddAssign + SubAssign + MulAssign + Copy> Matrix<K> {
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

    pub fn scl(&mut self, a: K){
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len(){
                self.data[i][j] *= a;
            }
        }
    }
}

impl<K: fmt::Display> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            for elem in row {
                write!(f, "{}", elem)?;  // O usa {:?} si quieres el formato de Vector
            }
            writeln!(f)?;
        }
        Ok(())
    }
}