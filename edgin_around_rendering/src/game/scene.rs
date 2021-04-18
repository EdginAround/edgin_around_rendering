use std::{cmp::Ordering, collections::HashMap, f32::consts::PI};

use crate::utils::{coordinates::Point, errors as err, ids::ActorId};

#[derive(Clone, Debug)]
pub enum TerrainVariant {
    Hills,
    Ranges,
    Continents,
}

#[derive(Clone, Debug)]
pub struct TerrainData {
    variant: TerrainVariant,
    origin: Point,
}

impl TerrainData {
    pub fn evaluate(&self, point: &Point, radius: f32) -> f32 {
        match self.variant {
            TerrainVariant::Hills => {
                0.006
                    * radius
                    * (point.theta / PI - 1.0)
                    * (50.0 * point.phi).sin()
                    * (point.theta / PI - 2.0)
                    * (50.0 * point.theta).sin()
            }
            TerrainVariant::Ranges => {
                0.012 * radius * (10.0 * point.theta + PI).cos() * (10.0 * point.phi).cos()
            }
            TerrainVariant::Continents => 0.018 * radius * point.theta.sin() * point.phi.sin(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ElevationFunction {
    radius: f32,
    terrain: Vec<TerrainData>,
}

impl ElevationFunction {
    pub fn new(radius: f32) -> Self {
        Self { radius, terrain: Vec::new() }
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn add_terrain(&mut self, name: &str, theta: f32, phi: f32) {
        let origin = Point::new(theta, phi);
        let variant = match name {
            "hills" => TerrainVariant::Hills,
            "ranges" => TerrainVariant::Ranges,
            "continents" => TerrainVariant::Continents,
            _ => return,
        };

        self.terrain.push(TerrainData { variant, origin });
    }

    pub fn evaluate(&self, point: &Point) -> f32 {
        let mut result = self.radius;
        for terrain in &self.terrain {
            result += terrain.evaluate(point, self.radius);
        }
        result
    }
}

impl Default for ElevationFunction {
    fn default() -> Self {
        Self { radius: 1000.0, terrain: Vec::new() }
    }
}

#[derive(Clone, Debug)]
pub struct Actor {
    id: ActorId,
    entity_name: String,
    position: Option<Point>,
}

impl Actor {
    pub fn new(id: ActorId, entity_name: String, position: Option<Point>) -> Self {
        Self { id, entity_name, position }
    }

    pub fn get_id(&self) -> ActorId {
        self.id
    }

    pub fn get_entity_name(&self) -> &str {
        self.entity_name.as_ref()
    }

    pub fn get_position(&self) -> Option<&Point> {
        self.position.as_ref()
    }

    pub fn is_visible(&self) -> bool {
        self.position.is_some()
    }

    pub fn set_position(&mut self, position: Point) {
        self.position = Some(position);
    }

    pub fn move_by(&mut self, distance: f32, bearing: f32, radius: f32) {
        if let Some(position) = &mut self.position {
            *position = position.moved_by(distance, bearing, radius);
        }
    }

    pub fn hide(&mut self) {
        self.position = None;
    }
}

#[derive(Clone, Debug)]
pub struct Scene {
    elevation: ElevationFunction,
    hero_actor_id: ActorId,
    actors: HashMap<ActorId, Actor>,
    is_ready: bool,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            elevation: ElevationFunction::default(),
            hero_actor_id: 0,
            actors: HashMap::new(),
            is_ready: false,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.is_ready
    }

    pub fn get_radius(&self) -> f32 {
        self.elevation.get_radius()
    }

    pub fn get_elevation(&self, point: &Point) -> f32 {
        self.elevation.evaluate(point)
    }

    pub fn get_hero_id(&self) -> ActorId {
        self.hero_actor_id
    }

    pub fn get_actor(&self, actor_id: ActorId) -> Option<&Actor> {
        self.actors.get(&actor_id)
    }

    pub fn get_actor_mut(&mut self, actor_id: ActorId) -> Option<&mut Actor> {
        self.actors.get_mut(&actor_id)
    }

    pub fn get_actors(&self) -> std::collections::hash_map::Values<ActorId, Actor> {
        self.actors.values()
    }

    pub fn get_focus_point(&self) -> Point {
        self.actors
            .get(&self.hero_actor_id)
            .cloned()
            .expect(err::NOT_EXISTING_HERO)
            .position
            .expect(err::HERO_WITHOUT_POSITION)
    }

    pub fn configure(&mut self, hero_actor_id: ActorId, elevation: ElevationFunction) {
        self.hero_actor_id = hero_actor_id;
        self.elevation = elevation;
        self.is_ready = true;
    }

    pub fn create_actors(&mut self, actors: &Vec<Actor>) {
        for actor in actors.iter() {
            self.actors.insert(actor.id, actor.clone());
        }
    }

    pub fn delete_actors(&mut self, actor_ids: &Vec<ActorId>) {
        for id in actor_ids {
            self.actors.remove(&id);
        }
    }

    pub fn hide_actors(&mut self, actor_ids: &Vec<ActorId>) {
        for id in actor_ids {
            if let Some(actor) = self.get_actor_mut(*id) {
                actor.hide();
            }
        }
    }

    pub fn find_closest_actors(
        &self,
        reference_position: &Point,
        max_distance: f32,
    ) -> Vec<ActorId> {
        let mut actors = Vec::<(ActorId, f32)>::new();
        for actor in self.actors.values() {
            if let Some(actor_position) = actor.get_position() {
                let current_distance =
                    Point::distance(reference_position, actor_position, self.get_radius());
                if current_distance < max_distance {
                    actors.push((actor.get_id(), current_distance));
                }
            }
        }

        actors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
        actors.iter().map(|a| a.0).collect()
    }
}
