use pyo3::prelude::*;

use edgin_around_rendering::utils::ids::ActorId;

#[pyclass]
#[derive(Clone, Debug)]
pub struct ElevationFunction {
    pub(crate) elevation_function: edgin_around_rendering::game::ElevationFunction,
}

#[pymethods]
impl ElevationFunction {
    #[new]
    pub fn new(radius: f32) -> Self {
        Self { elevation_function: edgin_around_rendering::game::ElevationFunction::new(radius) }
    }

    pub fn add_terrain(&mut self, name: &str, theta: f32, phi: f32) {
        self.elevation_function.add_terrain(name, theta, phi)
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct Actor {
    pub(crate) actor: edgin_around_rendering::game::Actor,
}

#[pymethods]
impl Actor {
    #[new]
    pub fn new(id: ActorId, entity_name: String, position: Option<crate::utils::Point>) -> Self {
        let point = position.map(|p| p.point);
        let actor = edgin_around_rendering::game::Actor::new(id, entity_name, point);
        Self { actor }
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct Scene {
    pub(crate) scene: edgin_around_rendering::game::Scene,
}

#[pymethods]
impl Scene {
    #[new]
    pub fn new() -> Self {
        Self { scene: edgin_around_rendering::game::Scene::new() }
    }

    pub fn configure(&mut self, hero_actor_id: ActorId, elevation: ElevationFunction) {
        self.scene.configure(hero_actor_id, elevation.elevation_function)
    }

    pub fn get_hero_id(&self) -> ActorId {
        self.scene.get_hero_id()
    }

    pub fn create_actors(&mut self, mut actors: Vec<Actor>) {
        let actors = actors.drain(..).map(|a| a.actor).collect();
        self.scene.create_actors(&actors)
    }

    pub fn delete_actors(&mut self, actor_ids: Vec<ActorId>) {
        self.scene.delete_actors(&actor_ids)
    }

    pub fn hide_actors(&mut self, actor_ids: Vec<ActorId>) {
        self.scene.hide_actors(&actor_ids)
    }

    pub fn get_radius(&self) -> f32 {
        self.scene.get_radius()
    }

    pub fn find_closest_actors(
        &self,
        reference_position: &crate::utils::Point,
        max_distance: f32,
    ) -> Vec<ActorId> {
        self.scene.find_closest_actors(&reference_position.point, max_distance)
    }

    pub fn set_actor_position(&mut self, actor_id: ActorId, position: crate::utils::Point) {
        if let Some(actor) = self.scene.get_actor_mut(actor_id) {
            actor.set_position(position.point);
        }
    }

    pub fn get_actor_position(
        &mut self,
        actor_id: ActorId,
    ) -> PyResult<Option<crate::utils::Point>> {
        if let Some(actor) = self.scene.get_actor(actor_id) {
            if let Some(point) = actor.get_position() {
                return Ok(Some(crate::utils::Point { point: point.clone() }));
            }
        }
        Ok(None)
    }
}
