use ndarray::{Array2, array};
use num_complex::Complex64;
use quantum_computing_experiments::utils::{dagger, kron};

type C = Complex64;

fn c(re: f64, im: f64) -> C {
    C::new(re, im)
}

fn approx_eq(a: &Array2<C>, b: &Array2<C>, eps: f64) -> bool {
    a.dim() == b.dim() && a.iter().zip(b.iter()).all(|(x, y)| (*x - *y).norm() < eps)
}

fn main() {
    let ket0 = array![[c(1.0, 0.0)], [c(0.0, 0.0)]];
    let bra0 = dagger(&ket0);

    let ket1 = array![[c(0.0, 0.0)], [c(1.0, 0.0)]];
    let bra1 = dagger(&ket1);

    let x = kron(&ket0, &bra1) + kron(&ket1, &bra0);

    println!("ket0:\n{ket0:?}");
    println!("bra0:\n{bra0:?}");
    println!("ket1:\n{ket1:?}");
    println!("bra1:\n{bra1:?}");
    println!("x:\n{x:?}");

    let pauli_x = array![[c(0.0, 0.0), c(1.0, 0.0)], [c(1.0, 0.0), c(0.0, 0.0)]];
    println!("pauliX:\n{pauli_x:?}");

    assert!(approx_eq(&x, &pauli_x, 1e-12));

    let state = array![[c(0.6, 0.0)], [c(0.8, 0.0)]];
    let transformed = x.dot(&state);

    println!("state:\n{state:?}");
    println!("x @ state:\n{transformed:?}");

    let expected = array![[c(0.8, 0.0)], [c(0.6, 0.0)]];
    assert!(approx_eq(&transformed, &expected, 1e-12));
}
