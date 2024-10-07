use pyo3::{prelude::*, types::PyDict};
use slint::Model;
use crate::ui::ZediBoxWindow;
use crate::ui::Device;

static mut ZEDI : Option<ZediBoxWindow> = None;

///this is our Gadget that python plugin code can create, and rust app can then access natively.
#[pyclass]
pub struct Gadget {
    #[pyo3(get, set)]
    pub prop: usize,
    //this field will only be accessible to rust code
    pub rustonly: Vec<usize>,
}

#[pymethods]
impl Gadget {
    #[new]
    fn new() -> Self {
        Gadget {
            prop: 777,
            rustonly: Vec::new(),
        }
    }

    fn push(&mut self, v: usize) {
        self.rustonly.push(v);
    }
}

#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("hello".into())
}

#[pyfunction]
fn get_all_devices() -> PyResult<String> {
    unsafe {
        match ZEDI {
            Some(ref zedi) => {
                let devices = zedi.get_devices();
                let x: Vec<Device> = devices.iter().map(|item| item).collect();
                let json_str: String = serde_json::to_string(&x).unwrap();
                return Ok(json_str.into());
            },
            None => {
                println!("ZediBoxWindow is not initialized");
                return Ok("".into());
            },
        };
    };
}

#[pyfunction]
fn log_info(msg: String)  -> PyResult<()> {
    println!("log: {}", msg);
    unsafe {
        match ZEDI {
            Some(ref zedi) => zedi.invoke_log_info(msg.into()),
            None => println!("ZediBoxWindow is not initialized"),
        }
    }
    
    Ok(())
}

/// A Python module for plugin interface types
#[pymodule]
pub fn plugin_api(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Gadget>()?;
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_function(wrap_pyfunction!(log_info, m)?)?;
    m.add_function(wrap_pyfunction!(get_all_devices, m)?)?;
    Ok(())
}

pub fn init_pybox(zedi: ZediBoxWindow){
    unsafe {
        ZEDI = Some(zedi);
    }
    pyo3::append_to_inittab!(plugin_api);
    pyo3::prepare_freethreaded_python();
}
pub fn run_py_code(code: &str) -> Result<&str, String> {
    let result = Python::with_gil(|py| {
        let globals = PyDict::new_bound(py);

        let result = py.run_bound(code, Some(&globals), None);
        result.map(|_| "Python code executed successfully.").map_err(|e| e.to_string())
    });
    result
}
