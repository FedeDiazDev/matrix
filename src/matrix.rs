use std::ops::{Add, Sub, Mul, Neg, AddAssign, SubAssign, MulAssign};
use std::fmt;
use crate::Vector;

#[derive(Clone, Debug)]
pub struct Matrix<K> {
    pub data: Vec<Vec<K>>,
}

impl<K> Matrix<K> {
    pub fn shape(&self) -> (usize, usize) {
        let rows = self.data.len();
        let cols = if rows == 0 { 0 } else { self.data[0].len() };
        (rows, cols)
    }
}

impl<K> Matrix<K>
where
    K: crate::Numeric,
{
    pub fn new(data: Vec<Vec<K>>) -> Self {
        if data.is_empty() || data[0].is_empty() {
            panic!("Matrix dimensions must be non-zero");
        }
        let cols = data[0].len();
        for (i, row) in data.iter().enumerate() {
            if row.len() != cols {
                panic!(
                    "Matrix row {} has a mismatched length of {} (expected {})",
                    i,
                    row.len(),
                    cols
                );
            }
        }
        Self { data }
    }

    pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K>
    where
        K: AddAssign,
    {
        let cols = if self.data.is_empty() { 0 } else { self.data[0].len() };
        if cols != vec.data.len() {
            panic!("Matrix columns must match vector length");
        }
        let mut result = Vec::new();
        for row in &self.data {
            let mut sum = K::default();
            for (&val, &v_val) in row.iter().zip(vec.data.iter()) {
                sum += val * v_val;
            }
            result.push(sum);
        }
        Vector::new(result)
    }

    pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K>
    where
        K: AddAssign,
    {
        let (rows_a, cols_a) = self.shape();
        let (rows_b, cols_b) = mat.shape();
        if cols_a != rows_b {
            panic!("Matrix A columns must match Matrix B rows");
        }
        let mut result = Vec::with_capacity(rows_a);

        for i in 0..rows_a {
            let mut new_row = Vec::with_capacity(cols_b);
            for j in 0..cols_b {
                let mut sum = K::default();
                for k in 0..cols_a {
                    sum += self.data[i][k] * mat.data[k][j];
                }
                new_row.push(sum);
            }
            result.push(new_row);
        }
        Matrix::new(result)
    }

    pub fn trace(&self) -> K
    where
        K: AddAssign,
    {
        let (rows, cols) = self.shape();
        if rows != cols {
            panic!("Matrix must be square");
        }
        let mut sum = K::default();
        for i in 0..rows {
            sum += self.data[i][i];
        }
        sum
    }

    pub fn transpose(&self) -> Matrix<K> {
        let (rows, cols) = self.shape();
        if rows == 0 || cols == 0 {
            return Matrix::new(Vec::new());
        }
        let mut result = Vec::with_capacity(cols);
        for j in 0..cols {
            let mut new_row = Vec::with_capacity(rows);
            for i in 0..rows {
                new_row.push(self.data[i][j]);
            }
            result.push(new_row);
        }
        Matrix::new(result)
    }
}

impl<K> Matrix<K>
where
    K: crate::Numeric + SubAssign,
{
    pub fn row_echelon(&self) -> Matrix<K> {
        let (rows, cols) = self.shape();
        if rows == 0 || cols == 0{
            return self.clone()
        }
        let mut res = self.clone();
        let mut pivot_row = 0;
        let zero = K::default();

        for col in 0..cols{
            if pivot_row == rows{
                break;
            }
            let mut selected_row = pivot_row;
            while selected_row < rows && res.data[selected_row][col] == zero {
                selected_row += 1;
            }
            if selected_row == rows {
                continue;
            }
            
            if selected_row != pivot_row{
                res.data.swap(selected_row, pivot_row);
            }
            let pivot_val = res.data[pivot_row][col];
            for j in 0..cols{
                res.data[pivot_row][j] = res.data[pivot_row][j] / pivot_val; 
            }
            for r in 0..rows{
                if r!= pivot_row{
                    let factor = res.data[r][col];
                    if factor != zero{
                        for j in 0..cols{
                            let sub = factor * res.data[pivot_row][j];
                            res.data[r][j] -= sub;
                        }
                    }
                }
            }
            pivot_row += 1;
        }
        for r in 0..rows {
            for c in 0..cols {
                if res.data[r][c] == zero {
                    res.data[r][c] = zero;
                }
            }
        }
        res
    }

    pub fn determinant(&self) -> K
    where
        K: Neg<Output = K>,
    {
        let (rows, cols) = self.shape();
        if rows != cols {
            panic!("Matrix must be square to compute its determinant");
        }
        if rows == 0 {
            return K::default();
        }
        if rows == 1 {
            return self.data[0][0];
        }

        let mut res = self.clone();
        let mut swapped = false;
        let zero = K::default();

        for i in 0..rows {
            let mut pivot_row = i;
            while pivot_row < rows && res.data[pivot_row][i] == zero {
                pivot_row += 1;
            }

            if pivot_row == rows {
                return zero;
            }

            if pivot_row != i {
                res.data.swap(pivot_row, i);
                swapped = !swapped;
            }

            let pivot = res.data[i][i];

            for r in (i + 1)..rows {
                let current_val = res.data[r][i];
                if current_val != zero {
                    let factor = current_val / pivot;
                    for j in i..cols {
                        let sub = factor * res.data[i][j];
                        res.data[r][j] -= sub;
                    }
                }
            }
        }
        let mut det = res.data[0][0];
        for i in 1..rows {
            det = det * res.data[i][i];
        }

        if swapped {
            -det
        } else {
            det
        }
    }
    pub fn inverse(&self) -> Result<Matrix::<K>, &'static str> {
        let (rows, cols) = self.shape();
        if rows != cols || rows == 0 {
            return Err("Matrix must be square and non-empty");
        }
        let zero = K::default();
        let mut found_non_zero = None;
        for r in 0..rows {
            for c in 0..cols {
                if self.data[r][c] != zero {
                    found_non_zero = Some(self.data[r][c]);
                    break;
                }
            }
            if found_non_zero.is_some() {
                break;
            }
        }

        let one = match found_non_zero {
            Some(val) => val / val,
            None => return Err("Matrix is singular (all elements are zero)"),
        };
        let mut inv_data = vec![vec![zero; cols]; rows];
        for i in 0..rows {
            inv_data[i][i] = one;
        }

        let mut mat = self.clone();
        let mut inv = Matrix::new(inv_data);

        for i in 0..rows {
            let mut pivot_row = i;
            while pivot_row < rows && mat.data[pivot_row][i] == zero {
                pivot_row += 1;
            }

            if pivot_row == rows {
                return Err("Matrix is singular (not invertible)");
            }

            if pivot_row != i {
                mat.data.swap(pivot_row, i);
                inv.data.swap(pivot_row, i);
            }

            let pivot_val = mat.data[i][i];
            for j in 0..cols {
                if mat.data[i][j] != zero {
                    mat.data[i][j] = mat.data[i][j] / pivot_val;
                }
                if inv.data[i][j] != zero {
                    inv.data[i][j] = inv.data[i][j] / pivot_val;
                }
            }

            for r in 0..rows {
                if r != i {
                    let factor = mat.data[r][i];
                    if factor != zero {
                        for j in 0..cols {
                            let sub_mat = factor * mat.data[i][j];
                            mat.data[r][j] -= sub_mat;

                            let sub_inv = factor * inv.data[i][j];
                            inv.data[r][j] -= sub_inv;
                        }
                    }
                }
            }
        }

        Ok(inv)
    }

    pub fn rank(&self) -> usize {
        let (rows, cols) = self.shape();
        if rows == 0 || cols == 0 {
            return 0;
        }

        let ref_mat = self.row_echelon();
        let zero = K::default();
        let mut rank = 0;

        for r in 0..rows {
            let is_non_zero_row = ref_mat.data[r].iter().any(|&val| val != zero);
            if is_non_zero_row {
                rank += 1;
            }
        }

        rank
    }

}

impl<K: AddAssign + Copy> Matrix<K> {
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
}

impl<K: SubAssign + Copy> Matrix<K> {
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
}

impl<K: MulAssign + Copy> Matrix<K> {
    pub fn scl(&mut self, a: K){
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len(){
                self.data[i][j] *= a;
            }
        }
    }
}

impl<K> Add for Matrix<K>
where
    K: AddAssign + Copy,
{
    type Output = Self;
    fn add(mut self, v: Self) -> Self {
        Matrix::add(&mut self, &v);
        self
    }
}

impl<K> Sub for Matrix<K>
where
    K: SubAssign + Copy,
{
    type Output = Self;
    fn sub(mut self, v: Self) -> Self {
        Matrix::sub(&mut self, &v);
        self
    }
}

impl<K> Mul<K> for Matrix<K>
where
    K: MulAssign + Copy,
{
    type Output = Self;
    fn mul(mut self, a: K) -> Self {
        self.scl(a);
        self
    }
}

impl<K, const R: usize, const C: usize> From<[[K; C]; R]> for Matrix<K>
where
    K: crate::Numeric
{
    fn from(data: [[K; C]; R]) -> Self {
        let mut vec_data = Vec::with_capacity(R);
        for row in data {
            vec_data.push(Vec::from(row));
        }
        Self::new(vec_data)
    }
}

impl<K: fmt::Display> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.data.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "[")?;
            for (j, elem) in row.iter().enumerate() {
                if j > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", elem)?;
            }
            write!(f, "]")?;
        }
        Ok(())
    }
}