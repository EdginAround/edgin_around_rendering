use std::f32::consts::PI;

use crate::{
    animations::Skeleton,
    game::Sprites,
    renderers::fixed::FixedRenderer,
    utils::{
        coordinates::Position,
        geometry::{Matrix3D, Vector3D},
        ids::{ActorId, MediumId},
    },
};

pub struct CameraPerspective {
    dist: f32,
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl Default for CameraPerspective {
    fn default() -> Self {
        Self { dist: 0.0, left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 }
    }
}

pub struct PositionedRenderer {
    actor_id: ActorId,
    renderer: FixedRenderer,
    position: Option<Position>,
    view: Matrix3D,
    model: Matrix3D,
    cam: CameraPerspective,
    highlight: bool,
}

impl PositionedRenderer {
    pub fn new(
        actor_id: ActorId,
        skeleton: Skeleton,
        skin_id: MediumId,
        position: Option<Position>,
        view: Matrix3D,
    ) -> Self {
        let mut mine = Self {
            actor_id: actor_id,
            renderer: FixedRenderer::new(skeleton, skin_id),
            position: None,
            view: Matrix3D::identity(),
            model: Matrix3D::identity(),
            cam: CameraPerspective::default(),
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
        self.calculate_screen_bounds();
    }

    pub fn change_view(&mut self, view: Matrix3D) {
        self.update_view(view);
        self.calculate_screen_bounds();
    }

    pub fn change_position_and_view(&mut self, position: Position, view: Matrix3D) {
        self.update_position(position);
        self.update_view(view);
        self.calculate_screen_bounds();
    }

    pub fn unset_position(&mut self) {
        self.position = None
    }

    pub fn select_animation(&mut self, name: &str) {
        self.renderer.select_animation(name);
    }

    pub fn is_visible(&self) -> bool {
        self.position.is_some()
    }

    pub fn get_camera_distance(&self) -> f32 {
        self.cam.dist
    }

    pub fn get_actor_id(&self) -> ActorId {
        self.actor_id
    }

    pub fn reacts_to(&self, x: f32, y: f32) -> bool {
        return (self.cam.left < x)
            && (x < self.cam.right)
            && (self.cam.bottom < y)
            && (y < self.cam.top);
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

    fn calculate_screen_bounds(&mut self) {
        let interaction = self.renderer.get_skeleton().get_interaction();
        let left = interaction.left;
        let right = interaction.right;
        let top = interaction.top;
        let bottom = interaction.bottom;

        let trans = &self.view * &self.model;
        let left_bottom = &trans * Vector3D::new(left, bottom, 0.0);
        let right_top = &trans * Vector3D::new(right, top, 0.0);
        let center = &trans * Vector3D::new(0.0, 0.0, 0.0);

        self.cam.left = left_bottom.get_x() / left_bottom.get_w();
        self.cam.bottom = left_bottom.get_y() / left_bottom.get_w();
        self.cam.right = right_top.get_x() / right_top.get_w();
        self.cam.top = right_top.get_y() / right_top.get_w();
        self.cam.dist = center.get_z() / center.get_w();
    }

    unsafe fn setup_rendering(&self, loc_highlight: gl::types::GLint, loc_model: gl::types::GLint) {
        gl::Uniform1i(loc_highlight, self.highlight as gl::types::GLint);
        gl::UniformMatrix4fv(loc_model, 1, gl::TRUE, self.model.as_ptr());
    }
}
