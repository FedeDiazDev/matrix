use std::ops::{AddAssign, MulAssign};
use matrix::{ Vector};

pub fn linear_combination<V, K: matrix::Numeric + MulAssign + AddAssign>(u: &[Vector<K>], coefs: &[K]) -> Vector<K>{
    if u.is_empty() || coefs.is_empty(){
        panic!("The vectors or coeficients cannot be empty");
    }
    if u.len() != coefs.len(){
        panic!("The vectors must have the same length");
    }
    let mut acum = Vector::new(u[0].data.clone());
    acum.scl(coefs[0]);
    for i in 1..u.len(){
        let mut temp = Vector::new(u[i].data.clone());
        temp.scl(coefs[i]);
        acum.add(&temp);
    }
    acum
}

pub fn main() {
    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);
    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);
    println!("{}", linear_combination::<Vector<f32>, f32>(&[e1, e2, e3], &[10., -2., 0.5]));
    // [10.]
    // [-2.]
    // [0.5]
    println!("------");
    println!("{}", linear_combination::<Vector<f32>, f32>(&[v1, v2], &[10., -2.]));
    // [10.]
    // [0.]
    // [230.]


}
