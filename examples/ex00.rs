use matrix::{Matrix, Vector};

fn main() {
    println!("=== VECTOR TESTS ===");

    println!("--- Vector Addition ---");
    let mut u = Vector::new(vec![]);
    let v = Vector::new(vec![5.0, 7.0]);
    u.add(&v);
    println!("Resultado:\n{}", u);

    println!("--- Vector Subtraction ---");
    let mut u = Vector::new(vec![2.0, 3.0]);
    let v = Vector::new(vec![5.0, 7.0]);
    u.sub(&v);
    println!("Resultado:\n{}", u);

    println!("--- Vector Scaling ---");
    let mut u = Vector::new(vec![2.0, 3.0]);
    u.scl(2.0);
    println!("Resultado:\n{}", u);

    println!("=== MATRIX TESTS ===");

    println!("--- Matrix Addition ---");
    let mut u = Matrix::new(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ]);
    let v = Matrix::new(vec![
        vec![7.0, 4.0],
        vec![3.0, 2.0],
    ]);
    u.add(&v);
    println!("Resultado:\n{}", u);
    println!("--- Matrix Subtraction ---");
    let mut u = Matrix::<f32>::new(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ]);
    let v = Matrix::<f32>::new(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ]);
    u.sub(&v);
    println!("Resultado:\n{}", u);

    println!("--- Matrix Scaling ---");
    let mut u = Matrix::new(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ]);
    u.scl(2.0);
    println!("Resultado:\n{}", u);
}