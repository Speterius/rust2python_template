extern crate cpython;
use cpython::{py_fn, py_module_initializer, PyResult, Python};

fn add(_: Python, a: i64, b: i64) -> PyResult<i64> {
    Ok(a + b)
}

py_module_initializer!(hello_rust, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "add", py_fn!(py, add(a: i64, b: i64)))?;
    Ok(())
});
