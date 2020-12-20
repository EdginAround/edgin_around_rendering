use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug)]
pub struct Point {
    pub(crate) point: edgin_around_rendering::utils::coordinates::Point,
}

#[pymethods]
impl Point {
    #[new]
    pub fn new(theta: f32, phi: f32) -> Self {
        Self { point: edgin_around_rendering::utils::coordinates::Point::new(theta, phi) }
    }

    pub fn get_theta(&self) -> f32 {
        self.point.theta
    }

    pub fn get_phi(&self) -> f32 {
        self.point.phi
    }
}
