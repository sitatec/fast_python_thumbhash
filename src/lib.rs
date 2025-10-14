use pyo3::prelude::*;
use thumbhash;

// Helper to convert the thumbhash-specific error into a Python ValueError.
fn to_py_err(e: thumbhash::ThumbHashError) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
}

#[pyfunction]
fn py_rgba_to_thumb_hash(py: Python, width: usize, height: usize, rgba: &[u8]) -> Vec<u8> {
    py.allow_threads(|| thumbhash::rgba_to_thumb_hash(width, height, rgba))
}

#[pyfunction]
fn py_thumb_hash_to_rgba(py: Python, hash: &[u8]) -> PyResult<(usize, usize, Vec<u8>)> {
    let result = py.allow_threads(|| {
        thumbhash::thumb_hash_to_rgba(hash)
    })?; 
    
    result.map_err(to_py_err)
}

#[pyfunction]
fn py_thumb_hash_to_average_rgba(py: Python, hash: &[u8]) -> (f32, f32, f32, f32) {
    py.allow_threads(|| thumbhash::thumb_hash_to_average_rgba(hash))
}

#[pyfunction]
fn py_thumb_hash_to_approximate_aspect_ratio(py: Python, hash: &[u8]) -> f32 {
    py.allow_threads(|| thumbhash::thumb_hash_to_approximate_aspect_ratio(hash))
}

#[pymodule]
fn fast_thumbhash(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_rgba_to_thumb_hash, m)?)?;
    m.add_function(wrap_pyfunction!(py_thumb_hash_to_rgba, m)?)?;
    m.add_function(wrap_pyfunction!(py_thumb_hash_to_average_rgba, m)?)?;
    m.add_function(wrap_pyfunction!(py_thumb_hash_to_approximate_aspect_ratio, m)?)?;
    Ok(())
}