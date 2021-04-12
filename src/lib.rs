extern crate pyo3;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn add(a: i64, b: i64) -> PyResult<i64> {
    Ok(a + b)
}

#[pyclass]
struct Rectangle {
    #[pyo3(get, set)]
    width: f64,

    #[pyo3(get)]
    height: f64,
}

#[pymethods]
impl Rectangle {
    #[new]
    pub fn new(w: f64, h: f64) -> Self {
        Rectangle {
            width: w,
            height: h,
        }
    }

    pub fn area(&self) -> PyResult<f64> {
        Ok(self.width * self.height)
    }
}

#[pymodule]
fn hello_rust(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_class::<Rectangle>()?;

    Ok(())
}
