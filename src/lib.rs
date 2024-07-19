use pyo3::prelude::*;

pub mod svdata;

#[pyfunction]
pub fn read_sv_file(file_path: &str) -> PyResult<svdata::SvData> {
    let defines = std::collections::HashMap::new();
    let includes: Vec<std::path::PathBuf> = Vec::new();

    let mut svdata = svdata::SvData {
        modules: Vec::new(),
    };

    if let Ok((syntax_tree, _)) = sv_parser::parse_sv(file_path, &defines, &includes, true, false) {
        sv_to_structure(&syntax_tree, file_path, &mut svdata);
    } else {
        Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Could not parse {file_path}."
        )))?;
    }

    Ok(svdata)
}

fn sv_to_structure(
    syntax_tree: &sv_parser::SyntaxTree,
    filepath: &str,
    svdata: &mut svdata::SvData,
) {
}

/// This module is implemented in Rust.
#[pymodule(name = "svdata")]
fn my_extension(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_sv_file, m)?)
}
