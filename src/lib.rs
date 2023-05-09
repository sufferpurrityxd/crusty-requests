mod request;
mod response;

use pyo3::prelude::*;

use request::get;

/// A Python module implemented in Rust.
#[pymodule]
fn crusty_requests(_py: Python, m: &PyModule) -> PyResult<()> {

    m.add_function(wrap_pyfunction!(get, m)?)?;
    Ok(())
}
