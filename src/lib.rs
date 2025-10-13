// src/lib.rs

use pyo3::prelude::*;
use thumbhash;

// Helper to convert a Rust panic inside an allow_threads block into a Python error.
// You can also use crates like `pyo3-log` or `pyo3-asyncio` which might have utilities for this.
fn to_py_err<E: std::error::Error>(e: E) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e))
}


#[pyfunction]
fn py_rgba_to_thumb_hash(py: Python, width: usize, height: usize, rgba: &[u8]) -> PyResult<Vec<u8>> {
    let result = py.allow_threads(|| {
        thumbhash::rgba_to_thumb_hash(width, height, rgba)
    });
    Ok(result)
}

#[pyfunction]
fn py_thumb_hash_to_rgba(py: Python, hash: &[u8]) -> PyResult<(usize, usize, Vec<u8>)> {
    let result = py.allow_threads(|| {
        thumbhash::thumb_hash_to_rgba(hash)
    });
    
    result.map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid thumbhash: {}", e))
    })
}

#[pyfunction]
fn py_thumb_hash_to_average_rgba(py: Python, hash: &[u8]) -> PyResult<(f32, f32, f32, f32)> {
    let result = py.allow_threads(|| {
        thumbhash::thumb_hash_to_average_rgba(hash)
    });
    Ok(result)
}

#[pyfunction]
fn py_thumb_hash_to_approximate_aspect_ratio(py: Python, hash: &[u8]) -> PyResult<f32> {
    let result = py.allow_threads(|| {
        thumbhash::thumb_hash_to_approximate_aspect_ratio(hash)
    });
    Ok(result)
}

#[pymodule]
fn fast_thumbhash(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_rgba_to_thumb_hash, m)?)?;
    m.add_function(wrap_pyfunction!(py_thumb_hash_to_rgba, m)?)?;
    m.add_function(wrap_pyfunction!(py_thumb_hash_to_average_rgba, m)?)?;
    m.add_function(wrap_pyfunction!(py_thumb_hash_to_approximate_aspect_ratio, m)?)?;
    Ok(())
}