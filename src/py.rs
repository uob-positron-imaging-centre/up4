use pyo3::prelude::*;
pub mod data;
pub mod converter;
use data::PyData;
use crate::base::PyGrid;
use converter::PyConverter;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
#[pyo3(name = "upppp_rust")]
fn upppp_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyData>()?;
    m.add_class::<PyGrid>()?;
    m.add_class::<PyConverter>()?;
    Ok(())
}