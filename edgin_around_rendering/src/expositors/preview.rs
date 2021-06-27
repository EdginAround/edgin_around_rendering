use crate::{
    animations, game, renderers,
    utils::{errors as err, geometry, graphics},
};

pub struct PreviewExpositor {
    sprites: game::Sprites,
    size: (usize, usize),
    renderer: renderers::FixedRenderer,
    program: gl::types::GLuint,
    loc_view: gl::types::GLint,
    loc_model: gl::types::GLint,
    view: geometry::Matrix3D,
    model: geometry::Matrix3D,
}

impl PreviewExpositor {
    pub fn new(
        sprite_path: &std::path::Path,
        skin_name: &str,
        saml_name: &str,
        variant_name: &str,
        action_name: &str,
        size: (usize, usize),
    ) -> Self {
        let saml_path = sprite_path.join(skin_name).join(saml_name);
        let parser = animations::Parser::new(&saml_path);
        let stock = parser.to_stock();

        let mut sprites = game::Sprites::new(sprite_path.into());
        let skin_id = sprites.load_skin(skin_name, &parser.get_sources());
        let sprite = animations::Sprite::new(skin_id, stock);

        let mut renderer = renderers::FixedRenderer::new(sprite);
        renderer.select_variant(variant_name);
        renderer.select_action(action_name);

        let program = graphics::prepare_entities_shader_program().expect(err::GL_SHADER_FAILED);
        let loc_view = graphics::get_uniform_location(program, "uniView".to_owned())
            .expect(err::GL_LOCATION_FAILED);
        let loc_model = graphics::get_uniform_location(program, "uniModel".to_string())
            .expect(err::GL_LOCATION_FAILED);

        let view = geometry::Matrix3D::identity();
        let model = geometry::Matrix3D::identity();

        Self { sprites, size, renderer, program, loc_view, loc_model, view, model }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.size = (width, height);
    }

    pub fn render(&mut self) {
        self.setup();
        self.draw();
        self.teardown();
    }
}

impl PreviewExpositor {
    fn setup(&self) {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::Enable(gl::DEPTH_TEST);

            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::DepthFunc(gl::LESS);

            gl::Viewport(0, 0, self.size.0 as gl::types::GLint, self.size.1 as gl::types::GLint);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    fn prepare_view(&self) -> geometry::Matrix3D {
        let (width, height) = (self.size.0 as f32, self.size.1 as f32);

        let (left, right, bottom, top) = {
            if width < height {
                (-0.6, 0.6, -0.1 * height / width, 1.1 * height / width)
            } else {
                (-0.6 * width / height, 0.6 * width / height, -0.1, 1.1)
            }
        };

        geometry::Matrix3D::orthographic(left, right, bottom, top, -100.0, 100.0)
    }

    fn draw(&mut self) {
        self.view = self.prepare_view();

        unsafe {
            gl::UseProgram(self.program);
            gl::UniformMatrix4fv(self.loc_view, 1, gl::TRUE, self.view.as_ptr());
            gl::UniformMatrix4fv(self.loc_model, 1, gl::TRUE, self.model.as_ptr());
        }

        self.renderer.render(&self.sprites);

        unsafe { gl::UseProgram(0) };
    }

    fn teardown(&self) {
        unsafe {
            gl::Disable(gl::BLEND);
            gl::Disable(gl::DEPTH_TEST);
        }
    }
}
