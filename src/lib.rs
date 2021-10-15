#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult};

fn multiples_simple(_py: Python, primes: Vec<f64>) -> PyResult<Vec<f64>> {
    let mut multiples = vec![1.];
    let max_val = (2.0_f64).powf(63.);
    for p in primes {
        let slice = &multiples[0..multiples.len()];
        let mut next = Vec::new();
        for &e in slice {
            let mut counter = 1.0;
            let mut prod = e * ((p as f64).powf(counter));
            while prod < max_val {
                next.push(prod);
                counter += 1.0;
                prod = e * ((p as f64).powf(counter));
            }
        }
        for &v in next.iter() {
            multiples.push(v);
        }
    }

    Ok(multiples)
}
py_module_initializer!(libmultiples, initlibmultiples, PyInit_libmultiples, |py, m | {
    m.add(py, "__doc__", "This module is implemented in Rust")?;
    m.add(py, "multiples", py_fn!(py, multiples_simple(val: Vec<f64>)))?;
    Ok(())
});