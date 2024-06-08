use graph_core;
use pyo3::{exceptions::PyValueError, prelude::*};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn graph_from_string(path: String) -> PyResult<String> {
    match graph_core::generate_graph(&path) {
        Ok(result) => Ok(result),
        Err(_) => Err(PyValueError::new_err("Error from graph")),
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn graph(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(graph_from_string, module)?)?;
    module.add_function(wrap_pyfunction!(sum_as_string, module)?)?;
    Ok(())
}
