use pyo3::prelude::*;

/// Basic metadata
#[pyclass]
#[derive(Clone, Debug)]
pub struct PlatformMetadata {
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub arch: String,
    #[pyo3(get, set)]
    pub desc: String,
}

#[pymethods]
impl PlatformMetadata {
    #[new]
    fn new(name: String, arch: String, desc: String) -> Self {
        PlatformMetadata {
            name,
            arch,
            desc,
        }
    }

    /// Custom `__repr__` method for better string representation in Python
    fn __repr__(&self) -> String {
        format!(
            "PlatformMetadata(name='{}', arch='{}', desc='{}')",
            self.name, self.arch, self.desc
        )
    }
}

/// Template for Platforms
#[pyclass]
pub struct Platform {
    py_impl: PyObject,
}

#[pymethods]
impl Platform {
    /// Public constructor expecting a Python implementation object.
    #[new]
    pub fn new(py_impl: PyObject) -> Self {
        Platform { py_impl }
    }

    /// Calls the Python side's "metadata" method and extracts a PlatformMetadata.
    pub fn metadata(&self, py: Python) -> PyResult<PlatformMetadata> {
        self.py_impl
            .call_method0(py, "metadata")?
            .extract(py)
    }

    /// Calls the Python side's "setup" method.
    pub fn setup(&self, py: Python) -> PyResult<()> {
        self.py_impl.call_method0(py, "setup")?;
        Ok(())
    }

    /// Calls the Python side's "set_driver" method with the given address and signal.
    pub fn set_driver(&self, py: Python, addr: u32, signal: u32) -> PyResult<()> {
        self.py_impl
            .call_method1(py, "set_driver", (addr, signal))?;
        Ok(())
    }

    /// Calls the Python side's "get_driver" method and extracts a u32.
    pub fn get_driver(&self, py: Python, addr: u32) -> PyResult<u32> {
        self.py_impl
            .call_method1(py, "get_driver", (addr,))?
            .extract(py)
    }

    /// Calls the Python side's "dispose" method.
    pub fn dispose(&self, py: Python) -> PyResult<()> {
        self.py_impl.call_method0(py, "dispose")?;
        Ok(())
    }
}

#[pymodule]
fn beatrice_v_api(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PlatformMetadata>()?;
    m.add_class::<Platform>()?;
    Ok(())
}
