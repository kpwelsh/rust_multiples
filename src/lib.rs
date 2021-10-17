#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult};

fn multiples_simple(_py: Python, primes: Vec<u64>, max: u64) -> PyResult<Vec<u64>> {
    let mut multiples = vec![primes.iter().product()];
    for &p in primes.iter() {
        let slice = &multiples[0..multiples.len()];
        let mut next = Vec::new();
        for &e in slice {
            let mut prod = e;
            while prod <= (max / p) {
                prod = prod * p;
                next.push(prod);
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
    m.add(py, "multiples", py_fn!(py, multiples_simple(val: Vec<u64>, max: u64)))?;
    Ok(())
});