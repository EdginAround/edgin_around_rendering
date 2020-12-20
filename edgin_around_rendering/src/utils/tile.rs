use crate::utils::{
    geometry::{Matrix2D, Vector2D},
    ids::ResourceId,
};

#[derive(Clone, Debug)]
pub struct Tile {
    pub id: ResourceId,
    pub points: [Vector2D; 4],
}

// TODO: Check if performance will improve if transformations are performed without a matrix.

impl Tile {
    pub fn new(id: ResourceId, position: (f32, f32), size: (f32, f32)) -> Self {
        let (x, y) = position;
        let (w, h) = size;

        Tile {
            id: id,
            points: [
                Vector2D::new(x, y),
                Vector2D::new(x, y + h),
                Vector2D::new(x + w, y + h),
                Vector2D::new(x + w, y),
            ],
        }
    }

    pub fn translate(&mut self, vector: (f32, f32)) {
        self.transform(&Matrix2D::translation(vector));
    }

    pub fn rotate(&mut self, angle: f32) {
        self.transform(&Matrix2D::rotation(angle));
    }

    pub fn scale(&mut self, vector: (f32, f32)) {
        self.transform(&Matrix2D::scale(vector));
    }

    pub fn transform(&mut self, matrix: &Matrix2D) {
        for point in self.points.iter_mut() {
            *point = matrix * point.clone();
        }
    }

    pub fn translated(&mut self, vector: (f32, f32)) -> Self {
        let mut other = self.clone();
        other.translate(vector);
        other
    }

    pub fn rotated(&mut self, angle: f32) -> Self {
        let mut other = self.clone();
        other.rotate(angle);
        other
    }

    pub fn scaled(&mut self, vector: (f32, f32)) -> Self {
        let mut other = self.clone();
        other.scale(vector);
        other
    }

    pub fn transformed(&mut self, matrix: &Matrix2D) -> Self {
        let mut other = self.clone();
        other.transform(matrix);
        other
    }
}
