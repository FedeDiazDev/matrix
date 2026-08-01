use std::ops::{AddAssign, SubAssign, MulAssign};
use std::fmt;

pub struct Vector<K> {
    pub data : Vec<K>
}

impl<K> Vector<K> {
    pub fn new(data: Vec<K>) -> Self {
        Self { data }
    }
}

impl<K: AddAssign + Copy> Vector<K> {
    // &mut self: Modifies the current vector (self) in place.
    // v: &Vector<K>: Immutable reference to the other vector (borrowed for reading without taking ownership).
     pub fn add(&mut self, v: &Vector<K>) { 
        for i in 0..v.data.len(){
            self.data[i] += v.data[i]
        }
     }
}

impl<K: SubAssign + Copy> Vector<K> {
    pub fn sub(&mut self, v: &Vector<K>) {
         for i in 0..v.data.len(){
            self.data[i] -= v.data[i]
        } 
    }
}

impl<K: MulAssign + Copy> Vector<K> {
    pub fn scl(&mut self, a: K) { 
        for i in 0..self.data.len(){
            self.data[i] *= a
        }
    }
}

impl<K, const N: usize> From<[K; N]> for Vector<K> {
    fn from(data: [K; N]) -> Self {
        Self::new(Vec::from(data))
    }
}

impl<K: fmt::Display> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, elem) in self.data.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "[{}]", elem)?;
        }
        Ok(())
    }
}