use std::ops::{Add, Sub, Mul, AddAssign, SubAssign, MulAssign};
use std::fmt;

#[derive(Clone, Debug)]
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
        if self.data.len() != v.data.len() {
            panic!("Vector dimensions must match");
        }
        for i in 0..v.data.len(){
            self.data[i] += v.data[i]
        }
     }
}

impl<K: SubAssign + Copy> Vector<K> {
    pub fn sub(&mut self, v: &Vector<K>) {
        if self.data.len() != v.data.len() {
            panic!("Vector dimensions must match");
        }
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

impl <K> Add for Vector<K>
where K: AddAssign + Copy
{
    type Output = Self;
    fn add(mut self, v: Self) -> Self {
        Vector::add(&mut self, &v);
        self
    }
}

impl <K>  Sub for Vector<K>
where K: SubAssign + Copy
{
    type Output = Self;
    fn sub(mut self, v: Self) -> Self {
        Vector::sub(&mut self, &v);
        self
    }
}

impl <K> Mul<K> for Vector<K>
where K: MulAssign + Copy
{
    type Output = Self;
    fn mul(mut self, a: K) -> Self {
        self.scl(a);
        self
    }
}

impl<K, const N: usize> From<[K; N]> for Vector<K> {
    fn from(data: [K; N]) -> Self {
        Self::new(Vec::from(data))
    }
}


impl <K> Vector<K> {
    pub fn dot(&self, v: &Self) -> K 
    where K: Copy + Mul<Output = K> + AddAssign + Default
    {
        if self.data.len() != v.data.len(){
            panic!("Vector dimensions must match")
        }
        let mut acc = K::default();
        for (&a, &b) in self.data.iter().zip(v.data.iter())
        {
            acc += a*b
        }
        acc
    }
}

impl Vector<f32> {
    pub fn norm_1(&self) -> f32 {
        self.data.iter().map(|x| x.abs()).sum()
    }
    pub fn norm(&self) -> f32 {
        self.dot(self).powf(0.5)
    }

    pub fn norm_inf(&self) -> f32 {
        self.data.iter().fold(0.0, |max, &x| max.max(x.abs()))
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