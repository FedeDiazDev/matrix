use matrix::Vector;

fn cross_product<K>(u:&Vector<K>, v:&Vector<K>) -> Vector<K>
where
    K: matrix::Numeric + std::ops::Sub<Output = K>,
{
    if u.data.len() != 3 || v.data.len() != 3{
        panic!("Vectors must have 3 dimensions");
    }
    let x = u.data[1]*v.data[2] - u.data[2]*v.data[1];
    let y = u.data[2]*v.data[0] - u.data[0]*v.data[2];
    let z = u.data[0]*v.data[1] - u.data[1]*v.data[0];
    Vector::from([x, y, z])
}

pub fn main () {
    let u = Vector::from([0., 0., 1.]);
    let v = Vector::from([1., 0., 0.]);
    println!("{}", cross_product(&u, &v));
    // [0.]
    // [1.]
    // [0.]
    println!("------");
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("{}", cross_product(&u, &v));
    // [-3.]
    // [6.]
    // [-3.]
    println!("------");
    let u = Vector::from([4., 2., -3.]);
    let v = Vector::from([-2., -5., 16.]);
    println!("{}", cross_product(&u, &v));
    // [17.]
    // [-58.]
    // [-16.]
}