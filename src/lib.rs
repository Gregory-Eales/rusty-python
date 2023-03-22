use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<u32> {
    Ok((a + b) as u32)
}



#[pyfunction]
fn get_n_primes(n: usize) -> PyResult<Vec<usize>> {

    let mut primes = vec![true; n+1];
    primes[0] = false;
    primes[1] = false;

    for i in 2..=(n as f64).sqrt() as usize {
        if primes[i] {
            for j in (i*i..=n).step_by(i) {
                primes[j] = false;
            }
        }
    }

    let mut result = Vec::new();
    for i in 2..=n {
        if primes[i] {
            result.push(i);
        }
    }
    Ok(result)
}



/// A Python module implemented in Rust.
#[pymodule]
fn rusty_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(get_n_primes, m)?)?;
    Ok(())
}