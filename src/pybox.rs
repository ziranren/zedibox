use pyo3::{prelude::*, types::PyDict};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("hello".into())
}

/// A Python module implemented in Rust.
#[pymodule]
fn zedi(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    Ok(())
}

pub fn run_py_code(code: &str) -> Result<&str, String> {
    pyo3::prepare_freethreaded_python();
    let result = Python::with_gil(|py| {
        let module = PyModule::new_bound(py, "zedi").unwrap();
        let wrapped_func = wrap_pyfunction!(hello, &module).unwrap();
        module.add_function(wrapped_func).unwrap();
        let globals = PyDict::new_bound(py);
        globals.set_item("zedi", module).unwrap();

        let result = py.run_bound(code, Some(&globals), None);
        result.map(|_| "Python code executed successfully.").map_err(|e| e.to_string())
    });
    result
}
