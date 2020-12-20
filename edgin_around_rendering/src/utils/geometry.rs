use ndarray;

#[derive(Clone, Debug)]
pub struct Vector2D {
    array: ndarray::Array2<f32>,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2D { array: ndarray::arr2(&[[x], [y], [1.0]]) }
    }

    pub fn get_x(&self) -> f32 {
        *self.array.get((0, 0)).unwrap()
    }

    pub fn get_y(&self) -> f32 {
        *self.array.get((1, 0)).unwrap()
    }

    pub fn get_w(&self) -> f32 {
        *self.array.get((2, 0)).unwrap()
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.array.as_ptr()
    }
}

#[derive(Debug)]
pub struct Matrix2D {
    array: ndarray::Array2<f32>,
}

impl Matrix2D {
    pub fn new(array: ndarray::Array2<f32>) -> Self {
        Matrix2D { array }
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.array.as_ptr()
    }
}

impl Matrix2D {
    pub fn identity() -> Self {
        #[rustfmt::skip]
        Matrix2D::new(ndarray::arr2(&[
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ]))
    }

    pub fn translation(vector: (f32, f32)) -> Self {
        let (x, y) = vector;

        #[rustfmt::skip]
        Matrix2D::new(ndarray::arr2(&[
            [1.0, 0.0,   x],
            [0.0, 1.0,   y],
            [0.0, 0.0, 1.0],
        ]))
    }

    pub fn rotation(angle: f32) -> Self {
        let (c, s) = (angle.cos(), angle.sin());

        #[rustfmt::skip]
        Matrix2D::new(ndarray::arr2(&[
            [  c,  -s, 0.0],
            [  s,   c, 0.0],
            [0.0, 0.0, 1.0],
        ]))
    }

    pub fn scale(scale: (f32, f32)) -> Self {
        let (x, y) = scale;

        #[rustfmt::skip]
        Matrix2D::new(ndarray::arr2(&[
            [  x, 0.0, 0.0],
            [0.0,   y, 0.0],
            [0.0, 0.0, 1.0],
        ]))
    }
}

impl std::ops::Mul<Vector2D> for Matrix2D {
    type Output = Vector2D;

    fn mul(self, rhs: Vector2D) -> Self::Output {
        Vector2D { array: self.array.dot(&rhs.array) }
    }
}

impl std::ops::Mul<&Vector2D> for Matrix2D {
    type Output = Vector2D;

    fn mul(self, rhs: &Vector2D) -> Self::Output {
        Vector2D { array: self.array.dot(&rhs.array) }
    }
}

impl std::ops::Mul<Vector2D> for &Matrix2D {
    type Output = Vector2D;

    fn mul(self, rhs: Vector2D) -> Self::Output {
        Vector2D { array: self.array.dot(&rhs.array) }
    }
}

impl std::ops::Mul<&Vector2D> for &Matrix2D {
    type Output = Vector2D;

    fn mul(self, rhs: &Vector2D) -> Self::Output {
        Vector2D { array: self.array.dot(&rhs.array) }
    }
}

impl std::ops::Mul<Matrix2D> for Matrix2D {
    type Output = Matrix2D;

    fn mul(self, rhs: Matrix2D) -> Self::Output {
        Matrix2D { array: self.array.dot(&rhs.array) }
    }
}

impl std::ops::Mul<&Matrix2D> for Matrix2D {
    type Output = Matrix2D;

    fn mul(self, rhs: &Matrix2D) -> Self::Output {
        Matrix2D { array: self.array.dot(&rhs.array) }
    }
}

impl std::ops::Mul<Matrix2D> for &Matrix2D {
    type Output = Matrix2D;

    fn mul(self, rhs: Matrix2D) -> Self::Output {
        Matrix2D { array: self.array.dot(&rhs.array) }
    }
}

impl std::ops::Mul<&Matrix2D> for &Matrix2D {
    type Output = Matrix2D;

    fn mul(self, rhs: &Matrix2D) -> Self::Output {
        Matrix2D { array: self.array.dot(&rhs.array) }
    }
}

#[derive(Clone, Debug)]
pub struct Vector3D {
    array: ndarray::Array2<f32>,
}

impl Vector3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3D { array: ndarray::arr2(&[[x], [y], [z], [1.0]]) }
    }

    pub fn get_x(&self) -> f32 {
        *self.array.get((0, 0)).unwrap()
    }

    pub fn get_y(&self) -> f32 {
        *self.array.get((1, 0)).unwrap()
    }

    pub fn get_z(&self) -> f32 {
        *self.array.get((2, 0)).unwrap()
    }

    pub fn get_w(&self) -> f32 {
        *self.array.get((3, 0)).unwrap()
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.array.as_ptr()
    }
}

#[derive(Clone, Debug)]
pub struct Matrix3D {
    array: ndarray::Array2<f32>,
}

impl Matrix3D {
    pub fn new(array: ndarray::Array2<f32>) -> Self {
        Matrix3D { array }
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.array.as_ptr()
    }
}

impl Matrix3D {
    pub fn identity() -> Self {
        #[rustfmt::skip]
        Matrix3D::new(ndarray::arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    pub fn perspective(fovy: f32, width: f32, height: f32, near: f32, far: f32) -> Self {
        let s = 1.0 / (0.5 * fovy).tan();
        let (sx, sy) = (s * height / width, s);
        let zz = (far + near) / (near - far);
        let zw = 2.0 * far * near / (near - far);

        #[rustfmt::skip]
        Matrix3D::new(ndarray::arr2(&[
            [ sx, 0.0,  0.0, 0.0],
            [0.0,  sy,  0.0, 0.0],
            [0.0, 0.0,   zz,  zw],
            [0.0, 0.0, -1.0, 0.0],
        ]))
    }

    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let wr = 1.0 / (right - left);
        let hr = 1.0 / (top - bottom);
        let dr = 1.0 / (far - near);

        #[rustfmt::skip]
        Matrix3D::new(ndarray::arr2(&[
            [2.0 * wr,      0.0,       0.0, -(right + left) * wr],
            [     0.0, 2.0 * hr,       0.0, -(top + bottom) * hr],
            [     0.0,      0.0, -2.0 * dr,   -(far + near) * dr],
            [     0.0,      0.0,       0.0,                  1.0],
        ]))
    }

    pub fn translation(vector: (f32, f32, f32)) -> Self {
        let (x, y, z) = vector;

        #[rustfmt::skip]
        Matrix3D::new(ndarray::arr2(&[
            [1.0, 0.0, 0.0,   x],
            [0.0, 1.0, 0.0,   y],
            [0.0, 0.0, 1.0,   z],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    pub fn rotation_x(angle: f32) -> Self {
        let (c, s) = (angle.cos(), angle.sin());

        #[rustfmt::skip]
        Matrix3D::new(ndarray::arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0,   c,  -s, 0.0],
            [0.0,   s,   c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    pub fn rotation_y(angle: f32) -> Self {
        let (c, s) = (angle.cos(), angle.sin());

        #[rustfmt::skip]
        Matrix3D::new(ndarray::arr2(&[
            [  c, 0.0,   s, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [ -s, 0.0,   c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    pub fn rotation_z(angle: f32) -> Self {
        let (c, s) = (angle.cos(), angle.sin());

        #[rustfmt::skip]
        Matrix3D::new(ndarray::arr2(&[
            [  c,  -s, 0.0, 0.0],
            [  s,   c, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    pub fn scale(scale: (f32, f32, f32)) -> Self {
        let (x, y, z) = scale;

        #[rustfmt::skip]
        Matrix3D::new(ndarray::arr2(&[
            [  x, 0.0, 0.0, 0.0],
            [0.0,   y, 0.0, 0.0],
            [0.0, 0.0,   z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }
}

impl std::ops::Mul<Vector3D> for Matrix3D {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        Vector3D { array: self.array.dot(&rhs.array) }
    }
}

impl std::ops::Mul<&Vector3D> for Matrix3D {
    type Output = Vector3D;

    fn mul(self, rhs: &Vector3D) -> Self::Output {
        Vector3D { array: self.array.dot(&rhs.array) }
    }
}

impl std::ops::Mul<Vector3D> for &Matrix3D {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        Vector3D { array: self.array.dot(&rhs.array) }
    }
}

impl std::ops::Mul<&Vector3D> for &Matrix3D {
    type Output = Vector3D;

    fn mul(self, rhs: &Vector3D) -> Self::Output {
        Vector3D { array: self.array.dot(&rhs.array) }
    }
}

impl std::ops::Mul<Matrix3D> for Matrix3D {
    type Output = Matrix3D;

    fn mul(self, rhs: Matrix3D) -> Self::Output {
        Matrix3D { array: self.array.dot(&rhs.array) }
    }
}

impl std::ops::Mul<&Matrix3D> for Matrix3D {
    type Output = Matrix3D;

    fn mul(self, rhs: &Matrix3D) -> Self::Output {
        Matrix3D { array: self.array.dot(&rhs.array) }
    }
}

impl std::ops::Mul<Matrix3D> for &Matrix3D {
    type Output = Matrix3D;

    fn mul(self, rhs: Matrix3D) -> Self::Output {
        Matrix3D { array: self.array.dot(&rhs.array) }
    }
}

impl std::ops::Mul<&Matrix3D> for &Matrix3D {
    type Output = Matrix3D;

    fn mul(self, rhs: &Matrix3D) -> Self::Output {
        Matrix3D { array: self.array.dot(&rhs.array) }
    }
}
