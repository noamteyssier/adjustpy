use adjustp::Procedure;
use numpy::{PyArray1, IntoPyArray, PyReadonlyArrayDyn};
use pyo3::{prelude::*, exceptions::PyValueError};

/// Adjust p-values using the specified method.
///
/// # Arguments
/// * `pvalues` - A 1D array of p-values.
/// * `method` - A string representing the method used (options = "bh", "by", "bonferroni", "benjamini-hochberg", "benjamini-yekutieli").
#[pyfunction]
fn adjust<'py>(
    py: Python<'py>, 
    pvalues: PyReadonlyArrayDyn<f64>, 
    method: &str
) -> PyResult<&'py PyArray1<f64>> {
    let procedure = match method {
        "bh" => Procedure::BenjaminiHochberg,
        "by" => Procedure::BenjaminiYekutieli,
        "bonferroni" => Procedure::Bonferroni,
        "benjamini-hochberg" => Procedure::BenjaminiHochberg,
        "benjamini-yekutieli" => Procedure::BenjaminiYekutieli,
        _ => return Err(PyValueError::new_err("Unknown method name provided")),
    };
    let slice = pvalues.as_slice().unwrap();
    let qvalues = adjustp::adjust(slice, procedure);
    Ok(qvalues.into_pyarray(py))
}

/// A Python module implemented in Rust.
#[pymodule]
fn adjustpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(adjust, m)?)?;
    Ok(())
}
