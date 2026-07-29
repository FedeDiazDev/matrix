use matrix::{Matrix, Vector};

fn main() {
    println!("=== VECTOR TESTS ===");

    println!("--- Vector Addition ---");
    let mut u = Vector::new(vec![2.0, 3.0]);
    let v = Vector::new(vec![5.0, 7.0]);
    u.add(&v);
    println!("Resultado:\n{}", u);
    println!("Esperado: [7.0, 10.0]\n");

    println!("--- Vector Subtraction ---");
    let mut u = Vector::new(vec![2.0, 3.0]);
    let v = Vector::new(vec![5.0, 7.0]);
    u.sub(&v);
    println!("Resultado:\n{}", u);
    println!("Esperado: [-3.0, -4.0]\n");

    println!("--- Vector Scaling ---");
    let mut u = Vector::new(vec![2.0, 3.0]);
    u.scl(2.0);
    println!("Resultado:\n{}", u);
    println!("Esperado: [4.0, 6.0]\n");

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
    println!("Esperado:\n[8.0, 6.0]\n[6.0, 6.0]\n");

    println!("--- Matrix Subtraction ---");
    let mut u = Matrix::new(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ]);
    let v = Matrix::new(vec![
        vec![7.0, 4.0],
        vec![3.0, 2.0],
    ]);
    u.sub(&v);
    println!("Resultado:\n{}", u);
    println!("Esperado:\n[-6.0, -2.0]\n[0.0, 2.0]\n");

    println!("--- Matrix Scaling ---");
    let mut u = Matrix::new(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ]);
    u.scl(2.0);
    println!("Resultado:\n{}", u);
    println!("Esperado:\n[2.0, 4.0]\n[6.0, 8.0]\n");
}