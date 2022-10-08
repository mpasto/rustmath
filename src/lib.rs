//use std::{vec, collections::HashMap};

use num_bigint::BigUint;
use num_traits::pow;
use pyo3::prelude::*;


fn simple_sieve(n : usize) -> Vec<usize> {

    if n == 1 { vec![]}
    else {
        let mut primes_bool = vec![true; n as usize];

        primes_bool[0] = false;
        primes_bool[1] = false;

        for i in (4..n).step_by(2) {
            primes_bool[i] = false;
        }

        for i in 3..((n as f64).sqrt().ceil() as usize) {
            if primes_bool[i] {
                for j in (i*i..n).step_by(2*i) {
                    primes_bool[j] = false;
                }
            }
        }

        let mut primes = Vec::<usize>::new();

        for (i,is_prime) in primes_bool.iter().enumerate() {
            if *is_prime {
                primes.push(i);
            }
        }


        primes
    }
}

fn segmented_sieve(n: usize) -> Vec<usize> {

    //1. Use simple sieve to find all primes up to sqrt(n)

    //2. Divide the interval [0..n-1] in each segment sqrt(n)

    //3. For each segment, create an array of size high-low+1
    //iterate through all primes

    let limit_simple = (n as f64).sqrt().ceil() as usize + 1;

    let mut primes = simple_sieve(limit_simple);

    let init_len = primes.len();


    for low in (limit_simple..n).step_by(limit_simple) {

        //for each chunk

        let mut high = low + limit_simple;
        if high > n {
            high = n;
        }

        let mut mark = vec![true; limit_simple];

        for i in 0..init_len {
            let prime = primes[i];
            //find the minimal multiple of prime
            let mut lo_lim = (((low as f64)/(prime as f64)).floor() as usize)*prime;

            if lo_lim < low {
                lo_lim += prime;
            }

            for j in (lo_lim..high).step_by(prime) {
                mark[j-low] = false;
            }
        }

        for j in low..high {
            if mark[j-low] {
                primes.push(j);
            }
        }
    }
    primes
}


fn adapted_sieve(n:usize) -> Vec<usize> {
    if n < 100000000 {
        simple_sieve(n)
    } else {
        segmented_sieve(n)
    }
}

fn sum_primes(n: usize) -> BigUint {
    let mut res = BigUint::new(vec![0]);
    for prime in adapted_sieve(n).iter(){
        res+= *prime
    }
    res
}

/// Return multiplicity of prime number "prime" as a factor of "n"
fn multiplicity(n: usize, prime: usize) -> usize {
    let mut q = n;
    let mut m = 0_usize;

    if prime > n {
        0_usize
    } else if prime > n/2 {
        1_usize
    } else {
        while q >= prime {
            q /= prime;
            m += q;
        }
        m
    }
}


fn factorial(n: usize) -> BigUint {

    let primes = simple_sieve(n+1);


    let mut result = BigUint::new(vec![1]);

    for prime in primes {
        let p_raised = pow(BigUint::new(vec![prime as u32]),multiplicity(n, prime));
        result *= p_raised;
    }
    
    result
}



/// Computes primes up to n using Eratosthene's sieve.
#[pyfunction]
fn compute_primes(n: usize) -> Vec<usize> {
    simple_sieve(n)
}

#[pyfunction]
fn compute_primes_sum(n: usize) -> BigUint {
    sum_primes(n)
}

#[pyfunction]
fn compute_factorial(n: usize) -> BigUint {
    factorial(n)
}

/// A Python math module for primes and factorials implemented in Rust.
#[pymodule]
fn rustmath(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute_primes, m)?)?;
    m.add_function(wrap_pyfunction!(compute_factorial, m)?)?;
    m.add_function(wrap_pyfunction!(compute_primes_sum, m)?)?;
    Ok(())
}