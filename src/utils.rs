use ndarray::Array2;
use num_complex::Complex64;
use std::ops::Mul;

pub fn dagger(matrix: &Array2<Complex64>) -> Array2<Complex64> {
    matrix.t().mapv(|z| z.conj())
}

pub fn kron<T>(a: &Array2<T>, b: &Array2<T>) -> Array2<T>
where
    T: Copy + Default + Mul<Output = T>,
{
    let (a_rows, a_cols) = a.dim();
    let (b_rows, b_cols) = b.dim();
    let mut out = Array2::<T>::default((a_rows * b_rows, a_cols * b_cols));

    for i in 0..a_rows {
        for j in 0..a_cols {
            let aij = a[(i, j)];
            for k in 0..b_rows {
                for l in 0..b_cols {
                    out[(i * b_rows + k, j * b_cols + l)] = aij * b[(k, l)];
                }
            }
        }
    }

    out
}
