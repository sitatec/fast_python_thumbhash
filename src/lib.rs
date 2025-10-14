use pyo3::prelude::*;
use thumbhash;

// Helper function to convert any error that implements `std::error::Error` into a `PyValueError`.
// This is good practice and makes the code cleaner.
fn to_py_err<E: std::error::Error>(e: E) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
}

#[pyfunction]
fn py_rgba_to_thumb_hash(py: Python, width: usize, height: usize, rgba: &[u8]) -> PyResult<Vec<u8>> {
    // allow_threads returns a PyResult. Use '?' to handle potential panics.
    // The inner function returns Vec<u8> directly, so the types match up.
    let result = py.allow_threads(|| {
        thumbhash::rgba_to_thumb_hash(width, height, rgba)
    });
    Ok(result)
}

#[pyfunction]
fn py_thumb_hash_to_rgba(py: Python, hash: &[u8]) -> PyResult<(usize, usize, Vec<u8>)> {
    // First, handle the outer Result from allow_threads (for panics)
    let result = py.allow_threads(|| {
        thumbhash::thumb_hash_to_rgba(hash)
    });

    // Now, handle the inner Result from the thumbhash function itself
    // result is a Result<Result<(usize, usize, Vec<u8>), ThumbHashError>, PyErr>
    // We can flatten this. Or handle it step-by-step.
    
    match result {
        // No panic occurred
        Ok(inner_result) => {
            // Now handle the Result from thumbhash::thumb_hash_to_rgba
            inner_result.map_err(to_py_err)
        }
        // A panic occurred inside the thread
        Err(e) => Err(e),
    }
}

#[pyfunction]
fn py_thumb_hash_to_average_rgba(py: Python, hash: &[u8]) -> PyResult<(f32, f32, f32, f32)> {
    // The thumbhash function returns the tuple directly, not a Result.
    // So allow_threads gives us a Result<(f32, ...), PyErr>, which is a PyResult.
    // We can just return it directly.
    py.allow_threads(|| {
        thumbhash::thumb_hash_to_average_rgba(hash)
    })
}

#[pyfunction]
fn py_thumb_hash_to_approximate_aspect_ratio(py: Python, hash: &[u8]) -> PyResult<f32> {
    // Same as above, the inner function returns f32, so allow_threads returns PyResult<f32>.
    py.allow_threads(|| {
        thumbhash::thumb_hash_to_approximate_aspect_ratio(hash)
    })
}

#[pymodule]
fn fast_thumbhash(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_rgba_to_thumb_hash, m)?)?;
    m.add_function(wrap_pyfunction!(py_thumb_hash_to_rgba, m)?)?;
    m.add_function(wrap_pyfunction!(py_thumb_hash_to_average_rgba, m)?)?;
    m.add_function(wrap_pyfunction!(py_thumb_hash_to_approximate_aspect_ratio, m)?)?;
    Ok(())
}