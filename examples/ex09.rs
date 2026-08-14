use matrix::Matrix;
pub fn main () {
    let u = Matrix::from([
    [1., 0., 0.],
    [0., 1., 0.],
    [0., 0., 1.],
    ]);
    println!("{}", u.transpose());
    println!("-----");
    // [1., 0., 0.]
    // [0., 1., 0.]
    // [0., 0., 1.]
    let u = Matrix::from([
    [1., 2., 3.],
    [4., 5., 6.],
    ]);
    println!("{}", u.transpose());
    println!("-----");
    // [1., 4.]
    // [2., 5.]
    // [3., 6.]
    let u = Matrix::from([
    [1., 2.],
    [3., 4.],
    [5., 6.],
    ]);
    println!("{}", u.transpose());
}