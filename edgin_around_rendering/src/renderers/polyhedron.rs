use gl;

use crate::utils::{defs::prelude::*, figures, ids::TextureId};

pub struct PolyhedronRenderer {
    texture_id: TextureId,
    index_count: usize,
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    ibo: gl::types::GLuint,
}

impl PolyhedronRenderer {
    pub fn new(texture_id: TextureId, figure: figures::Polyhedron) -> Self {
        let mut vertices =
            Vec::<gl::types::GLfloat>::with_capacity(3 * figure.get_vertices().len());
        for vertex in figure.get_vertices() {
            vertices.push(vertex.x);
            vertices.push(vertex.y);
            vertices.push(vertex.z);
        }

        let mut indices = Vec::<gl::types::GLuint>::with_capacity(3 * figure.get_triangles().len());
        for triangle in figure.get_triangles() {
            indices.push(triangle.0);
            indices.push(triangle.1);
            indices.push(triangle.2);
        }

        let mut mine = Self { texture_id, index_count: indices.len(), vao: 0, vbo: 0, ibo: 0 };

        unsafe {
            gl::GenVertexArrays(1, &mut mine.vao);
            gl::GenBuffers(1, &mut mine.vbo);
            gl::GenBuffers(1, &mut mine.ibo);

            gl::BindVertexArray(mine.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, mine.vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mine.ibo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                SIZEPTR_FLOAT * vertices.len() as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                SIZEPTR_FLOAT * indices.len() as gl::types::GLsizeiptr,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        mine
    }

    pub fn render(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, NULL);

            gl::DrawElements(
                gl::TRIANGLES,
                SIZE_FLOAT * self.index_count as gl::types::GLint,
                gl::UNSIGNED_INT,
                NULL,
            );

            gl::DisableVertexAttribArray(0);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
}

impl Default for PolyhedronRenderer {
    fn default() -> Self {
        let vertices = Vec::<gl::types::GLfloat>::with_capacity(0);
        let indices = Vec::<gl::types::GLuint>::with_capacity(0);

        let mut mine = Self { texture_id: 0, index_count: 0, vao: 0, vbo: 0, ibo: 0 };

        unsafe {
            gl::GenVertexArrays(1, &mut mine.vao);
            gl::GenBuffers(1, &mut mine.vbo);
            gl::GenBuffers(1, &mut mine.ibo);

            gl::BindVertexArray(mine.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, mine.vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mine.ibo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                0 as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                0 as gl::types::GLsizeiptr,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        mine
    }
}

impl Drop for PolyhedronRenderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo as *const _);
            gl::DeleteBuffers(1, &self.ibo as *const _);
        }
    }
}
