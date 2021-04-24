use std::collections::HashMap;

use crate::{
    animations::skeleton::{Animation, Image, Skeleton, ANIMATION_NAME_DEFAULT},
    utils::{
        errors as err,
        geometry::Matrix2D,
        ids::{MediumId, ResourceId},
        tile::Tile,
    },
};

type Subsprites = HashMap<String, Sprite>;

struct TileInfo<'a> {
    transformation: Matrix2D,
    image_id: Option<MediumId>,
    bone_name: &'a str,
}

impl<'a> TileInfo<'a> {
    pub fn new(transformation: Matrix2D, image_id: Option<MediumId>, bone_name: &'a str) -> Self {
        Self { transformation, image_id, bone_name }
    }
}

#[derive(Clone, Debug)]
pub struct Sprite {
    selected_animation: String,
    skeleton: Skeleton,
    skin_id: MediumId,
    subsprites: Subsprites,
}

impl Sprite {
    pub fn new(skeleton: Skeleton, skin_id: MediumId) -> Self {
        let selected_animation = ANIMATION_NAME_DEFAULT.to_string();
        let subsprites = Subsprites::new();
        Sprite { selected_animation, skeleton, skin_id, subsprites }
    }

    pub fn get_selected_animation(&self) -> &Animation {
        self.skeleton
            .get_animation(self.selected_animation.as_str())
            .expect(err::SAML_NOT_EXISTING_ANIMATION)
    }

    pub fn get_selected_animation_name(&self) -> &str {
        &self.selected_animation
    }

    pub fn select_animation(&mut self, name: &str) -> Result<(), ()> {
        if self.skeleton.has_animation(name) {
            self.selected_animation = name.to_string();
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn select_default_animation(&mut self) -> Result<(), ()> {
        self.select_animation(ANIMATION_NAME_DEFAULT)
    }

    pub fn get_animation_duration(&self) -> f32 {
        self.get_selected_animation().get_duration()
    }

    pub fn is_looped(&self) -> bool {
        self.get_selected_animation().is_looped()
    }

    pub fn get_max_num_layers(&self) -> usize {
        self.skeleton.get_max_num_layers()
    }

    pub fn attach_sprite(&mut self, hook_name: String, sprite: Sprite) {
        self.subsprites.insert(hook_name, sprite);
    }

    pub fn detach_sprite(&mut self, hook_name: &str) {
        self.subsprites.remove(hook_name);
    }

    pub fn tick(&self, moment: f32) -> Vec<Tile> {
        let animation = self.get_selected_animation();
        let scale = Matrix2D::scale(animation.get_scale());
        let unscale = Matrix2D::scale(animation.get_scale_reversed());

        // Prepare bone poses. Bones are sorted in such a way that parents are always prepared
        // before their children.
        // TODO: Avoid memory allocation.
        let mut infos = Vec::<TileInfo>::with_capacity(animation.get_num_layers());
        for bone in animation.get_bones() {
            let (mut trans, image_id) = bone.calc_state_at(moment, animation.get_duration());
            if let Some(parent_index) = bone.get_parent_index() {
                let parent_info = infos.get(parent_index).unwrap();
                trans = &parent_info.transformation * trans;
            }
            infos.push(TileInfo::new(trans, image_id, bone.get_name()));
        }

        // Prepare tiles.
        let mut tiles = Vec::<Tile>::new();
        for &index in animation.get_draw_order().iter() {
            let info = infos.get(index).unwrap();

            if let Some(sprite) = &self.subsprites.get(info.bone_name) {
                let transformation = &scale * &info.transformation * &unscale;
                let mut subtiles = sprite.tick(moment);
                for subtile in &mut subtiles {
                    subtile.transform(&transformation);
                }
                tiles.extend_from_slice(&subtiles);
            } else if let Some(image_id) = info.image_id {
                let resource_id = ResourceId::new(self.skin_id, image_id);
                let scale = animation.get_scale();
                let image = self.skeleton.get_image(image_id).expect(err::SAML_NOT_EXISTING_IMAGE);
                tiles.push(self.make_tile(resource_id, image, &info.transformation).scaled(scale));
            }
        }
        tiles
    }
}

impl Sprite {
    fn make_tile(&self, resource_id: ResourceId, image: &Image, trans: &Matrix2D) -> Tile {
        let image_pivot = image.get_pivot();
        let image_size = image.get_size();
        let tile_pivot = (-image_pivot.0, image_pivot.1 - image_size.1);
        let mut tile = Tile::new(resource_id, tile_pivot, image_size);
        tile.transform(trans);
        tile
    }
}
