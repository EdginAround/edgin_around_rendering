use std::f32::consts::PI;

use crate::{
    animations::Sprite,
    game::Sprites,
    renderers::fixed::FixedRenderer,
    utils::{
        coordinates::Position,
        geometry::{Matrix3D, Vector3D},
        ids::ActorId,
    },
};

pub struct PositionedRenderer {
    actor_id: ActorId,
    renderer: FixedRenderer,
    position: Option<Position>,
    view: Matrix3D,
    model: Matrix3D,
    camera_distance: f32,
    highlight: bool,
}

impl PositionedRenderer {
    pub fn new(
        actor_id: ActorId,
        sprite: Sprite,
        position: Option<Position>,
        view: Matrix3D,
    ) -> Self {
        let mut mine = Self {
            actor_id: actor_id,
            renderer: FixedRenderer::new(sprite),
            position: None,
            view: Matrix3D::identity(),
            model: Matrix3D::identity(),
            camera_distance: 0.0,
            highlight: false,
        };

        if let Some(position) = position {
            mine.change_position_and_view(position, view);
        }

        mine
    }

    pub fn set_highlight(&mut self, highlight: bool) {
        self.highlight = highlight;
    }

    pub fn change_position(&mut self, position: Position) {
        self.update_position(position);
        self.calculate_camera_distance();
    }

    pub fn change_view(&mut self, view: Matrix3D) {
        self.update_view(view);
        self.calculate_camera_distance();
    }

    pub fn change_position_and_view(&mut self, position: Position, view: Matrix3D) {
        self.update_position(position);
        self.update_view(view);
        self.calculate_camera_distance();
    }

    pub fn unset_position(&mut self) {
        self.position = None
    }

    pub fn get_sprite(&self) -> &Sprite {
        self.renderer.get_sprite()
    }

    pub fn get_sprite_mut(&mut self) -> &mut Sprite {
        self.renderer.get_sprite_mut()
    }

    pub fn select_animation(&mut self, name: &str) {
        self.renderer.select_animation(name);
    }

    pub fn has_position(&self) -> bool {
        self.position.is_some()
    }

    pub fn get_camera_distance(&self) -> f32 {
        self.camera_distance
    }

    pub fn get_actor_id(&self) -> ActorId {
        self.actor_id
    }

    pub fn render(
        &mut self,
        loc_highlight: gl::types::GLint,
        loc_model: gl::types::GLint,
        sprites: &Sprites,
    ) {
        unsafe { self.setup_rendering(loc_highlight, loc_model) };
        self.renderer.render(sprites);
    }
}

impl PositionedRenderer {
    fn update_position(&mut self, position: Position) {
        self.model = Matrix3D::rotation_x(-0.5 * PI)
            * Matrix3D::rotation_z(position.phi)
            * Matrix3D::rotation_x(position.theta)
            * Matrix3D::rotation_z(-position.bearing)
            * Matrix3D::translation((0.0, 0.0, position.altitude))
            * Matrix3D::rotation_x(0.5 * PI);
        self.position = Some(position);
    }

    fn update_view(&mut self, view: Matrix3D) {
        self.view = view
    }

    fn calculate_camera_distance(&mut self) {
        let center = &self.view * &self.model * Vector3D::new(0.0, 0.0, 0.0);
        self.camera_distance = center.get_z() / center.get_w();
    }

    unsafe fn setup_rendering(&self, loc_highlight: gl::types::GLint, loc_model: gl::types::GLint) {
        gl::Uniform1i(loc_highlight, self.highlight as gl::types::GLint);
        gl::UniformMatrix4fv(loc_model, 1, gl::TRUE, self.model.as_ptr());
    }
}
