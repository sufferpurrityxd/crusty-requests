use pyo3::prelude::*;
use crate::response::Response;

#[pyfunction]
pub fn get(url: String, py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        Ok(Response::from(isahc::get_async(url.clone()).await.unwrap()))
    })
}
