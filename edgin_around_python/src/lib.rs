use pyo3::{prelude::*, wrap_pyfunction};

pub mod expositors;
pub mod game;
pub mod utils;

#[pyfunction]
fn init() {
    edgin_around_rendering::init().expect("Initialize EdginAround");
}

/// A Python module implemented in Rust.
#[pymodule]
fn edgin_around_rendering(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init, m)?)?;

    m.add_class::<utils::Point>()?;

    m.add_class::<game::Actor>()?;
    m.add_class::<game::ElevationFunction>()?;
    m.add_class::<game::Scene>()?;

    m.add_class::<expositors::PreviewExpositor>()?;
    m.add_class::<expositors::WorldExpositor>()?;

    Ok(())
}
