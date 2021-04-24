use pyo3::prelude::*;

use edgin_around_rendering::utils::{
    defs::{Radian, Zoom},
    ids::ActorId,
};

#[pyclass]
pub struct WorldExpositor {
    pub(crate) world: edgin_around_rendering::expositors::WorldExpositor,
}

#[pymethods]
impl WorldExpositor {
    #[new]
    pub fn new(resource_dir: &str, size: (usize, usize)) -> Self {
        let resource_path = std::path::PathBuf::from(resource_dir);
        let world = edgin_around_rendering::expositors::WorldExpositor::new(resource_path, size);
        Self { world }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.world.resize(width, height)
    }

    pub fn render(&mut self, scene: &crate::game::Scene) {
        self.world.render(&scene.scene)
    }
}

#[pymethods]
impl WorldExpositor {
    pub fn get_bearing(&self) -> Radian {
        self.world.get_bearing()
    }

    pub fn get_highlighted_actor_id(&self) -> Option<ActorId> {
        self.world.get_highlighted_actor_id()
    }

    pub fn set_highlighted_actor_id(&mut self, actor_id: ActorId) {
        self.world.set_highlighted_actor_id(Some(actor_id))
    }

    pub fn remove_highlight(&mut self) {
        self.world.set_highlighted_actor_id(None)
    }

    pub fn zoom_by(&mut self, zoom: Zoom) {
        self.world.zoom_by(zoom)
    }

    pub fn rotate_by(&mut self, angle: Radian) {
        self.world.rotate_by(angle)
    }

    pub fn tilt_by(&mut self, angle: Radian) {
        self.world.tilt_by(angle)
    }

    pub fn create_renderers(&mut self, mut actors: Vec<crate::game::Actor>) {
        let actors = actors.drain(..).map(|a| a.actor).collect();
        self.world.create_renderers(&actors)
    }

    pub fn delete_renderers(&mut self, ids: Vec<ActorId>) {
        self.world.delete_renderers(&ids)
    }

    pub fn play_animation(&mut self, actor_id: ActorId, animation_name: String) {
        self.world.play_animation(actor_id, &animation_name)
    }

    pub fn attach_actor(
        &mut self,
        hook_name: String,
        base_actor_id: ActorId,
        attached_actor_id: Option<ActorId>,
    ) {
        self.world.attach_actor(hook_name, base_actor_id, attached_actor_id)
    }
}

#[pyclass]
pub struct PreviewExpositor {
    pub(crate) preview: edgin_around_rendering::expositors::PreviewExpositor,
}

#[pymethods]
impl PreviewExpositor {
    #[new]
    pub fn new(
        sprite_dir: &str,
        skin_name: &str,
        saml_name: &str,
        animation_name: &str,
        size: (usize, usize),
    ) -> Self {
        let sprite_path = std::path::Path::new(sprite_dir);
        let preview = edgin_around_rendering::expositors::PreviewExpositor::new(
            sprite_path,
            skin_name,
            saml_name,
            animation_name,
            size,
        );
        Self { preview }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.preview.resize(width, height)
    }

    pub fn render(&mut self) {
        self.preview.render()
    }
}
