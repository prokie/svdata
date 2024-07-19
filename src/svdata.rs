use pyo3::prelude::*;

#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub struct SvData {
    #[pyo3(get, set)]
    pub modules: Vec<String>,
}

#[pymethods]
impl SvData {
    #[new]
    fn new() -> Self {
        SvData {
            modules: Vec::new(),
        }
    }
}
