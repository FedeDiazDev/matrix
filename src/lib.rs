pub mod matrix;
pub mod vector;

pub use matrix::Matrix;
pub use vector::Vector;

pub trait Numeric:
    Copy
    + Default
    + PartialEq
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
{}

impl<T> Numeric for T where
    T: Copy
        + Default
        + PartialEq
        + std::ops::Add<Output = Self>
        + std::ops::Sub<Output = Self>
        + std::ops::Mul<Output = Self>
        + std::ops::Div<Output = Self>
{}
