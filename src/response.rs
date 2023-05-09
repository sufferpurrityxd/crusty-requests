use pyo3::prelude::*;
use std::sync::Arc;

use isahc::{AsyncBody, AsyncReadResponseExt, ResponseExt};
use isahc::http::HeaderValue;
use tokio::sync::Mutex;

pub struct ResponseCore {
    pub inner: isahc::Response<AsyncBody>,
}

#[pyclass]
pub struct Response(pub Arc<Mutex<ResponseCore>>);

impl From<isahc::Response<AsyncBody>> for Response {
    fn from(value: isahc::Response<AsyncBody>) -> Self {
        let response_core = ResponseCore { inner: value };
        return Self(Arc::new(Mutex::new(response_core)));
    }
}

#[pymethods]
impl Response {
    #[getter]
    pub fn text<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let c = Arc::clone(&self.0);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let text = c.lock().await.inner.text().await.unwrap();
            Ok(text)
        })
    }

    #[getter]
    pub fn status<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let c = Arc::clone(&self.0);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Ok(c.lock().await.inner.status().as_u16())
        })
    }

    #[getter]
    pub fn json<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let c = Arc::clone(&self.0);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let json = c
                .lock()
                .await
                .inner
                .json::<serde_json::Value>()
                .await
                .unwrap();
            Ok(json.to_string())
        })
    }

    #[getter]
    pub fn remote_addr<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let c = Arc::clone(&self.0);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Ok(c.lock().await.inner.remote_addr().unwrap().to_string())
        })
    }

}
