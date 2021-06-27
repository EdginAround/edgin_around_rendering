use std::collections::HashMap;

use crate::{
    animations::{
        skeleton::{Animation, Image, ACTION_NAME_DEFAULT, VARIANT_NAME_DEFAULT},
        stock::Stock,
    },
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
    skin_id: MediumId,
    stock: Stock,
    selected_animation: Animation,
    subsprites: Subsprites,
    selected_variant_name: String,
    selected_action_name: String,
}

impl Sprite {
    pub fn new(skin_id: MediumId, stock: Stock) -> Self {
        let animation_id = stock
            .select(VARIANT_NAME_DEFAULT, ACTION_NAME_DEFAULT)
            .expect(err::DEFAULT_VARIANT_AND_ACTION_FAILED);

        let animation =
            stock.get_animation(animation_id).expect(err::DEFAULT_ANIMATION_FAILED).clone();

        Sprite {
            skin_id,
            stock,
            selected_animation: animation,
            subsprites: Subsprites::new(),
            selected_variant_name: VARIANT_NAME_DEFAULT.to_string(),
            selected_action_name: ACTION_NAME_DEFAULT.to_string(),
        }
    }

    pub fn get_animation_duration(&self) -> f32 {
        self.selected_animation.get_duration()
    }

    pub fn is_looped(&self) -> bool {
        self.selected_animation.is_looped()
    }

    pub fn get_max_num_layers(&self) -> usize {
        self.stock.get_max_num_layers()
    }

    pub fn attach_sprite(&mut self, hook_name: String, sprite: Sprite) {
        self.subsprites.insert(hook_name, sprite);
    }

    pub fn detach_sprite(&mut self, hook_name: &str) {
        self.subsprites.remove(hook_name);
    }

    pub fn get_selected_variant_name(&self) -> &String {
        &self.selected_variant_name
    }

    pub fn get_selected_action_name(&self) -> &String {
        &self.selected_action_name
    }

    pub fn select_variant(&mut self, variant_name: &str) -> Result<(), ()> {
        if self.selected_variant_name == variant_name {
            return Ok(());
        }

        let animation_id = self.stock.select(variant_name, &self.selected_action_name);
        if let Some(animation_id) = animation_id {
            if let Some(animation) = self.stock.get_animation(animation_id) {
                self.selected_animation = animation.clone();
                self.selected_variant_name = variant_name.to_string();
                Ok(())
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn select_variant_or_default(&mut self, variant_name: &str) -> Result<(), ()> {
        if self.selected_variant_name == variant_name {
            return Ok(());
        }

        let mut animation_id = self.stock.select(variant_name, &self.selected_action_name);
        if animation_id.is_none() {
            animation_id = self.stock.select(VARIANT_NAME_DEFAULT, &self.selected_action_name);
        }

        if let Some(animation_id) = animation_id {
            if let Some(animation) = self.stock.get_animation(animation_id) {
                self.selected_animation = animation.clone();
                self.selected_variant_name = variant_name.to_string();
                Ok(())
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn select_action(&mut self, action_name: &str) -> Result<(), ()> {
        if self.selected_action_name == action_name {
            return Ok(());
        }

        let animation_id = self.stock.select(&self.selected_variant_name, action_name);
        if let Some(animation_id) = animation_id {
            if let Some(animation) = self.stock.get_animation(animation_id) {
                self.selected_animation = animation.clone();
                self.selected_action_name = action_name.to_string();
                Ok(())
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn select_action_or_default(&mut self, action_name: &str) -> Result<(), ()> {
        if self.selected_action_name == action_name {
            return Ok(());
        }

        let mut animation_id = self.stock.select(&self.selected_variant_name, action_name);
        if animation_id.is_none() {
            animation_id = self.stock.select(&self.selected_variant_name, ACTION_NAME_DEFAULT);
        }

        if let Some(animation_id) = animation_id {
            if let Some(animation) = self.stock.get_animation(animation_id) {
                self.selected_animation = animation.clone();
                self.selected_action_name = action_name.to_string();
                Ok(())
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn select_default_action(&mut self) -> Result<(), ()> {
        self.select_action(ACTION_NAME_DEFAULT)
    }

    pub fn tick(&self, moment: f32) -> Vec<Tile> {
        let animation = &self.selected_animation;
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
                let image = self.stock.get_image(image_id).expect(err::SAML_NOT_EXISTING_IMAGE);
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
