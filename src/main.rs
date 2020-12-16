use num::{BigUint, One};
use rayon::prelude::*;
use std::ops::Mul;
use std::time::Instant;

const S: u32 = 999999;

/// Compute the Factorial using a plain iterator.
fn _factorial(n: u32) -> BigUint {
    (1..n + 1).map(BigUint::from).fold(BigUint::one(), Mul::mul)
}

/// Benchmark the Factorial using a plain iterator.
fn _factorial_iterator(u: u32) {
    let start = Instant::now();
    let _result = _factorial(u);
    println!("Regular Fact:\t{:?}", start.elapsed());
}

/// Compute the Factorial using rayon::par_iter.
fn factorial_par_iter(u: u32) {
    fn fact(n: u32) -> BigUint {
        (1..n + 1)
            .into_par_iter()
            .map(BigUint::from)
            .reduce_with(Mul::mul)
            .unwrap()
    }
    let start = Instant::now();
    let _result = fact(u);
    println!("Factorial Par:\t{:?}", start.elapsed());
}

/// Compute the Factorial using divide-and-conquer serial recursion.
fn factorial_recursion(u: u32) {
    fn product(a: u32, b: u32) -> BigUint {
        if a == b {
            return a.into();
        }
        let mid = (a + b) / 2;
        product(a, mid) * product(mid + 1, b)
    }
    let start = Instant::now();
    let _result = product(1, u);
    println!("Factorial Rec.:\t{:?}", start.elapsed());
}

/// Compute the Factorial using divide-and-conquer parallel join.
fn factorial_join(u: u32) {
    fn product(a: u32, b: u32) -> BigUint {
        if a == b {
            return a.into();
        }
        let mid = (a + b) / 2;
        let (x, y) = rayon::join(|| product(a, mid), || product(mid + 1, b));
        x * y
    }

    let start = Instant::now();
    let _result = product(1, u);
    println!("Factorial Join:\t{:?}", start.elapsed());
}

fn main() {
    factorial_par_iter(S);
    factorial_recursion(S);
    factorial_join(S);
}
