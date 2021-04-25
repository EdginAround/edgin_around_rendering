use std::time::Instant;

use crate::{
    animations::{Sprite, ANIMATION_NAME_DEFAULT},
    game::Sprites,
    utils::{defs::prelude::*, defs::SIZE_FLOAT, errors as err, tile::Tile},
};

#[derive(Debug)]
pub struct FixedRenderer {
    sprite: Sprite,
    start_instant: Instant,
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    ibo: gl::types::GLuint,
}

impl FixedRenderer {
    pub fn new(sprite: Sprite) -> Self {
        let start_instant = Instant::now();
        let mut mine = Self { sprite, start_instant, vao: 0, vbo: 0, ibo: 0 };

        unsafe {
            gl::GenVertexArrays(1, &mut mine.vao);
            gl::GenBuffers(1, &mut mine.vbo);
            gl::GenBuffers(1, &mut mine.ibo);

            mine.bind();
            mine.load_indices();
            mine.unbind();
        }

        mine.select_animation(ANIMATION_NAME_DEFAULT);

        mine
    }

    pub fn get_sprite(&self) -> &Sprite {
        &self.sprite
    }

    pub fn get_sprite_mut(&mut self) -> &mut Sprite {
        &mut self.sprite
    }

    pub fn select_animation(&mut self, name: &str) {
        if name != self.sprite.get_selected_animation_name() {
            self.start_instant = Instant::now();
            if self.sprite.select_animation(name).is_err() {
                self.sprite.select_default_animation().expect(err::DEFAULT_ANIMATION_FAILED);
            }
        }
    }

    pub fn render(&mut self, sprites: &Sprites) {
        const LOC_POSITION: gl::types::GLuint = 0;
        const LOC_TEX_COORD: gl::types::GLuint = 1;
        const SIZE_POSITION: gl::types::GLint = 3;
        const SIZE_TEX_COORD: gl::types::GLint = 2;
        const PTR_POSITION: *const gl::types::GLvoid = 0 as _;
        const PTR_TEX_COORD: *const gl::types::GLvoid = (SIZE_POSITION * SIZE_FLOAT) as _;
        const STRIDE: gl::types::GLint = (SIZE_POSITION + SIZE_TEX_COORD) * SIZE_FLOAT;

        let mut duration = (Instant::now() - self.start_instant).as_secs_f32();
        if (!self.sprite.is_looped()) && (self.sprite.get_animation_duration() < duration) {
            self.sprite.select_default_animation().expect(err::DEFAULT_ANIMATION_FAILED);
            duration = 0.0;
        }

        unsafe {
            self.bind();

            let tiles = self.sprite.tick(duration);
            self.load_vertices(&tiles);

            gl::VertexAttribPointer(
                LOC_POSITION,
                SIZE_POSITION,
                gl::FLOAT,
                gl::FALSE,
                STRIDE,
                PTR_POSITION,
            );
            gl::EnableVertexAttribArray(LOC_POSITION);

            gl::VertexAttribPointer(
                LOC_TEX_COORD,
                SIZE_TEX_COORD,
                gl::FLOAT,
                gl::FALSE,
                STRIDE,
                PTR_TEX_COORD,
            );
            gl::EnableVertexAttribArray(LOC_TEX_COORD);

            for (i, tile) in tiles.iter().enumerate() {
                let j = i as i32;
                if let Some(texture_id) = sprites.get_texture_id(&tile.id) {
                    gl::BindTexture(gl::TEXTURE_2D, texture_id);
                    gl::DrawElements(
                        gl::TRIANGLES,
                        VERTICES_PER_RECT_INT,
                        gl::UNSIGNED_INT,
                        (SIZE_FLOAT * VERTICES_PER_RECT_INT * j) as *const _,
                    );
                }
            }

            gl::DisableVertexAttribArray(LOC_TEX_COORD);
            gl::DisableVertexAttribArray(LOC_POSITION);

            self.unbind();
        }
    }
}

impl FixedRenderer {
    unsafe fn bind(&self) {
        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
    }

    unsafe fn unbind(&self) {
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    unsafe fn load_vertices(&self, tiles: &Vec<Tile>) {
        let vertices = self.prepare_vertices(tiles);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            SIZEPTR_FLOAT * vertices.len() as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::DYNAMIC_DRAW,
        );
    }

    unsafe fn load_indices(&self) {
        let indices = self.prepare_indices();
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            SIZEPTR_FLOAT * indices.len() as gl::types::GLsizeiptr,
            indices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
    }

    fn prepare_vertices(&self, tiles: &Vec<Tile>) -> Vec<gl::types::GLfloat> {
        const SEPARATION: gl::types::GLfloat = 0.001;
        let mut data = Vec::with_capacity(20 * tiles.len());
        for (i, tile) in tiles.iter().enumerate() {
            // TODO: Calculate texture coordinates in the shader.
            let f = i as f32;
            data.append(&mut vec![
                tile.points[0].get_x(),
                tile.points[0].get_y(),
                SEPARATION * f,
                0.0,
                1.0,
                tile.points[1].get_x(),
                tile.points[1].get_y(),
                SEPARATION * f,
                0.0,
                0.0,
                tile.points[2].get_x(),
                tile.points[2].get_y(),
                SEPARATION * f,
                1.0,
                0.0,
                tile.points[3].get_x(),
                tile.points[3].get_y(),
                SEPARATION * f,
                1.0,
                1.0,
            ]);
        }
        data
    }

    fn prepare_indices(&self) -> Vec<gl::types::GLuint> {
        let num_layers = self.sprite.get_max_num_layers();
        let mut result = Vec::with_capacity(num_layers * VERTICES_PER_RECT_SIZE);
        for num in 0..num_layers as gl::types::GLuint {
            for offset in [0, 1, 2, 2, 3, 0].iter() {
                result.push(4 * num + offset);
            }
        }
        return result;
    }
}

impl Drop for FixedRenderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.ibo);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
