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

impl <K: AddAssign + SubAssign + MulAssign + Copy> Vector<K> {
    // &mut self: Modifies the current vector (self) in place.
    // v: &Vector<K>: Immutable reference to the other vector (borrowed for reading without taking ownership).
     pub fn add(&mut self, v: &Vector<K>) { 
        for i in 0..v.data.len(){
            self.data[i] += v.data[i]
        }
     }
    pub fn sub(&mut self, v: &Vector<K>) {
         for i in 0..v.data.len(){
            self.data[i] -= v.data[i]
        } 
    }
    pub fn scl(&mut self, a: K) { 
        for i in 0..self.data.len(){
            self.data[i] *= a
        }
    }
}

impl<K: fmt::Display> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, elem) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", elem)?;
        }
        write!(f, "]")
    }
}