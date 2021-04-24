use std::{cmp::Ordering, f32::consts::PI};

use crate::{
    animations, game, renderers,
    utils::{coordinates, defs, errors as err, figures, geometry, graphics, ids::ActorId},
};

const INITIAL_THETA: f32 = 0.0;
const INITIAL_PHI: f32 = 0.0;
const INITIAL_RADIUS: f32 = 0.0;
const INITIAL_ELEVATION: f32 = 0.0;
const INITIAL_ZOOM: defs::Zoom = 10.0;
const INITIAL_BEARING: defs::Radian = 0.0 * PI;
const INITIAL_TILT: defs::Radian = 0.4 * PI;
const ZOOM_BOUNDS: (defs::Zoom, defs::Zoom) = (0.0, 1000.0);
const TILT_BOUNDS: (defs::Radian, defs::Radian) = (0.1 * PI, 1.5 * PI);
const GROUND_QUALITY: u32 = 6;

pub struct WorldExpositor {
    resource_path: std::path::PathBuf,

    textures: game::Textures,
    sprites: game::Sprites,

    theta: f32,
    phi: f32,
    radius: f32,
    elevation: f32,
    zoom: defs::Zoom,
    bearing: defs::Radian,
    tilt: defs::Radian,

    size: (usize, usize),
    highlighted_actor_id: Option<ActorId>,

    program_ground: gl::types::GLuint,
    program_entities: gl::types::GLuint,
    loc_ground_view: gl::types::GLint,
    loc_entities_view: gl::types::GLint,
    loc_entities_model: gl::types::GLint,
    loc_entities_highlight: gl::types::GLint,

    renderer_ground: renderers::PolyhedronRenderer,
    renderer_water: renderers::PolyhedronRenderer,
    renderers_entities: Vec<renderers::PositionedRenderer>,

    view: geometry::Matrix3D,

    ready: bool,
}

impl WorldExpositor {
    pub fn new(resource_path: std::path::PathBuf, size: (usize, usize)) -> Self {
        let sprite_path = game::sprites_path(&resource_path);
        Self {
            resource_path: resource_path,
            textures: game::Textures::default(),
            sprites: game::Sprites::new(sprite_path),
            theta: INITIAL_THETA,
            phi: INITIAL_PHI,
            radius: INITIAL_RADIUS,
            elevation: INITIAL_ELEVATION,
            zoom: INITIAL_ZOOM,
            bearing: INITIAL_BEARING,
            tilt: INITIAL_TILT,
            size,
            highlighted_actor_id: None,
            program_ground: defs::UNONE,
            program_entities: defs::UNONE,
            loc_ground_view: defs::INONE,
            loc_entities_view: defs::INONE,
            loc_entities_model: defs::INONE,
            loc_entities_highlight: defs::INONE,
            renderer_ground: renderers::PolyhedronRenderer::default(),
            renderer_water: renderers::PolyhedronRenderer::default(),
            renderers_entities: Vec::new(),
            view: geometry::Matrix3D::identity(),
            ready: false,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.size = (width, height);
    }

    pub fn render(&mut self, scene: &game::Scene) {
        if !self.ready && scene.is_ready() {
            self.init_gl();
            self.load_data(scene);
            self.ready = true;
        }

        if self.ready {
            self.setup();
            self.draw(scene);
            self.teardown();
        }
    }
}

impl WorldExpositor {
    pub fn get_bearing(&self) -> defs::Radian {
        self.bearing
    }

    pub fn get_highlighted_actor_id(&self) -> Option<ActorId> {
        self.highlighted_actor_id
    }

    pub fn set_highlighted_actor_id(&mut self, actor_id: Option<ActorId>) {
        self.highlighted_actor_id = actor_id;
    }

    pub fn zoom_by(&mut self, zoom: defs::Zoom) {
        let new_zoom = self.zoom - zoom;

        if (ZOOM_BOUNDS.0 < new_zoom) && (new_zoom < ZOOM_BOUNDS.1) {
            self.zoom = new_zoom;
        }
    }

    pub fn rotate_by(&mut self, angle: defs::Radian) {
        self.bearing += angle;
        while self.bearing > PI {
            self.bearing -= 2.0 * PI;
        }
        while self.bearing < -PI {
            self.bearing += 2.0 * PI;
        }
    }

    pub fn tilt_by(&mut self, angle: defs::Radian) {
        let new_tilt = self.tilt + angle;

        if (TILT_BOUNDS.0 < new_tilt) && (new_tilt < TILT_BOUNDS.1) {
            self.tilt = new_tilt;
        }
    }

    pub fn create_renderers(&mut self, actors: &Vec<game::Actor>) {
        for actor in actors.iter() {
            let sprite = self.load_sprite(actor.get_entity_name());

            // TODO: Cache loaded skeletons.
            let position = if let Some(point) = actor.get_position() {
                Some(coordinates::Position::new(point.theta, point.phi, self.bearing, self.radius))
            } else {
                None
            };

            let renderer = renderers::PositionedRenderer::new(
                actor.get_id(),
                sprite,
                position,
                geometry::Matrix3D::identity(),
            );

            self.renderers_entities.push(renderer);
        }
    }

    pub fn delete_renderers(&mut self, ids: &Vec<ActorId>) {
        self.renderers_entities.drain_filter(|renderer| ids.contains(&renderer.get_actor_id()));
    }

    pub fn play_animation(&mut self, actor_id: ActorId, animation_name: &str) {
        if let Some(renderer) = self.find_renderer(actor_id) {
            renderer.select_animation(animation_name);
        }
    }

    pub fn attach_actor(
        &mut self,
        hook_name: String,
        target_actor_id: ActorId,
        source_actor_id: Option<ActorId>,
    ) {
        let source_sprite: Option<animations::Sprite> =
            source_actor_id.and_then(|id| self.find_renderer(id)).map(|r| r.get_sprite().clone());

        if let Some(target_renderer) = self.find_renderer(target_actor_id) {
            let target_sprite = target_renderer.get_sprite_mut();
            if let Some(mut source_sprite) = source_sprite {
                source_sprite
                    .select_animation(animations::ANIMATION_NAME_HELD)
                    .expect(err::SAML_NOT_EXISTING_ANIMATION);
                target_sprite.attach_sprite(hook_name, source_sprite);
            } else {
                target_sprite.detach_sprite(&hook_name);
            }
        }
    }
}

impl WorldExpositor {
    fn setup(&self) {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::Enable(gl::DEPTH_TEST);

            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::DepthFunc(gl::LESS);

            gl::Viewport(0, 0, self.size.0 as gl::types::GLint, self.size.1 as gl::types::GLint);

            gl::ClearColor(0.6, 0.7, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    fn draw(&mut self, scene: &game::Scene) {
        // Get hero position
        self.update_lookat(scene);

        // Refresh transformation
        self.view = self.prepare_view();

        // Draw ground
        unsafe {
            gl::UseProgram(self.program_ground);
            gl::UniformMatrix4fv(self.loc_ground_view, 1, gl::TRUE, self.view.as_ptr());
        }

        self.renderer_water.render();
        self.renderer_ground.render();

        // Update entities with bearing and sort them by distance from the camera
        for renderer in self.renderers_entities.iter_mut() {
            let actor = scene.get_actor(renderer.get_actor_id()).expect(err::NOT_EXISTING_ACTOR);
            renderer.set_highlight(Some(renderer.get_actor_id()) == self.highlighted_actor_id);

            if let Some(position) = actor.get_position() {
                let elevation = scene.get_elevation(position);
                let position = coordinates::Position::new(
                    position.theta,
                    position.phi,
                    self.bearing,
                    elevation,
                );
                renderer.change_position_and_view(position, self.view.clone());
            } else {
                renderer.unset_position();
            }
        }
        self.renderers_entities.sort_by(|a, b| {
            b.get_camera_distance().partial_cmp(&a.get_camera_distance()).unwrap_or(Ordering::Equal)
        });

        // Draw entities
        unsafe {
            gl::UseProgram(self.program_entities);
            gl::UniformMatrix4fv(self.loc_entities_view, 1, gl::TRUE, self.view.as_ptr());
        }

        for renderer in self.renderers_entities.iter_mut() {
            if renderer.has_position() {
                renderer.render(
                    self.loc_entities_highlight,
                    self.loc_entities_model,
                    &self.sprites,
                );
            }
        }

        unsafe { gl::UseProgram(0) };
    }

    fn teardown(&self) {
        unsafe {
            gl::Disable(gl::BLEND);
            gl::Disable(gl::DEPTH_TEST);
        }
    }
}

impl WorldExpositor {
    fn init_gl(&mut self) {
        self.program_ground =
            graphics::prepare_ground_shader_program().expect(err::GL_SHADER_FAILED);
        self.program_entities =
            graphics::prepare_entities_shader_program().expect(err::GL_SHADER_FAILED);

        unsafe {
            gl::UseProgram(self.program_ground);
            self.loc_ground_view =
                graphics::get_uniform_location(self.program_ground, "uniView".to_owned())
                    .expect(err::GL_LOCATION_FAILED);

            gl::UseProgram(self.program_entities);
            self.loc_entities_view =
                graphics::get_uniform_location(self.program_entities, "uniView".to_owned())
                    .expect(err::GL_LOCATION_FAILED);
            self.loc_entities_model =
                graphics::get_uniform_location(self.program_entities, "uniModel".to_owned())
                    .expect(err::GL_LOCATION_FAILED);
            self.loc_entities_highlight =
                graphics::get_uniform_location(self.program_entities, "uniHighlight".to_owned())
                    .expect(err::GL_LOCATION_FAILED);

            gl::UseProgram(0)
        }
    }

    fn load_data(&mut self, scene: &game::Scene) {
        self.radius = scene.get_radius();
        self.elevation = scene.get_elevation(&coordinates::Point::new(self.theta, self.phi));

        let mut ground = figures::sphere(GROUND_QUALITY, self.radius);
        let water = ground.clone();
        ground.rescale(|x, y| {
            let point = coordinates::Point::new(x, y);
            scene.get_elevation(&point)
        });

        self.textures = game::Textures::load(&self.resource_path);
        self.renderer_water = renderers::PolyhedronRenderer::new(self.textures.water, water);
        self.renderer_ground = renderers::PolyhedronRenderer::new(self.textures.grass, ground);
    }
}

impl WorldExpositor {
    fn load_sprite(&mut self, name: &str) -> animations::Sprite {
        // TODO: Load only if needed.
        let saml_path = self.sprites.get_sprites_dir().join(name).join(name).with_extension("saml");
        let parser = animations::Parser::new(&saml_path);
        let skin_id = self.sprites.load_skin_if_needed(name, &parser.get_sources());
        animations::Sprite::new(parser.to_skeleton(), skin_id)
    }

    fn update_lookat(&mut self, scene: &game::Scene) {
        let position = scene.get_focus_point();
        self.theta = position.theta;
        self.phi = position.phi;
        self.elevation = scene.get_elevation(&position);
    }

    fn prepare_view(&self) -> geometry::Matrix3D {
        geometry::Matrix3D::perspective(
            0.25 * PI,
            self.size.0 as f32,
            self.size.1 as f32,
            1.0,
            100.0,
        ) * geometry::Matrix3D::translation((0.0, 0.0, -self.zoom))
            * geometry::Matrix3D::rotation_x(-self.tilt)
            * geometry::Matrix3D::translation((0.0, 0.0, -self.elevation))
            * geometry::Matrix3D::rotation_z(self.bearing)
            * geometry::Matrix3D::rotation_x(-self.theta)
            * geometry::Matrix3D::rotation_z(-self.phi)
            * geometry::Matrix3D::rotation_x(0.5 * PI)
    }

    fn find_renderer(&mut self, actor_id: ActorId) -> Option<&mut renderers::PositionedRenderer> {
        for renderer in self.renderers_entities.iter_mut() {
            if renderer.get_actor_id() == actor_id {
                return Some(renderer);
            }
        }
        None
    }
}
