use gl;

pub type Zoom = f32;
pub type Radian = f32;

pub const NULL: *const gl::types::GLvoid = std::ptr::null();
pub const INONE: gl::types::GLint = 0;
pub const UNONE: gl::types::GLuint = 0;
pub const SIZE_FLOAT: gl::types::GLint =
    std::mem::size_of::<gl::types::GLfloat>() as gl::types::GLint;
pub const SIZEPTR_FLOAT: gl::types::GLsizeiptr =
    std::mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizeiptr;
pub const VERTICES_PER_RECT_INT: gl::types::GLint = 6;
pub const VERTICES_PER_RECT_SIZE: usize = 6;

pub const DEFAULT_ANIMATION: &str = "idle";

pub mod prelude {
    pub use super::{
        DEFAULT_ANIMATION, INONE, NULL, SIZEPTR_FLOAT, SIZE_FLOAT, UNONE, VERTICES_PER_RECT_INT,
        VERTICES_PER_RECT_SIZE,
    };
}
