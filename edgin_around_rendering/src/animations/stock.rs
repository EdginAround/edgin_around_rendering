use std::collections::HashMap;

use crate::{
    animations::skeleton::{Animation, Image, ACTION_NAME_DEFAULT, VARIANT_NAME_DEFAULT},
    utils::ids::MediumId,
};

type Selection = HashMap<String, HashMap<String, String>>;

#[derive(Clone, Debug)]
pub struct Stock {
    animations: HashMap<String, Animation>,
    selection: Selection,
    images: Vec<Image>,
    max_num_layers: usize,
}

impl Stock {
    pub fn new(
        animations: HashMap<String, Animation>,
        selection: Selection,
        images: Vec<Image>,
    ) -> Self {
        let max_num_layers =
            animations.values().map(|animation| animation.get_num_layers()).max().unwrap_or(0);

        Self { animations, selection, images, max_num_layers }
    }

    pub fn get_animation(&self, animation_id: &str) -> Option<&Animation> {
        self.animations.get(animation_id)
    }

    pub fn select(&self, variant_name: &str, action_name: &str) -> Option<&String> {
        let actions = if let Some(actions) = self.selection.get(variant_name) {
            actions
        } else if let Some(actions) = self.selection.get(VARIANT_NAME_DEFAULT) {
            actions
        } else {
            log::error!(
                "Neither {:?} nor {:?} variant could be found",
                variant_name,
                VARIANT_NAME_DEFAULT
            );
            return None;
        };

        let action_id = if let Some(action_id) = actions.get(action_name) {
            Some(action_id)
        } else {
            actions.get(ACTION_NAME_DEFAULT)
        };

        action_id
    }

    pub fn get_image(&self, image_id: MediumId) -> Option<&Image> {
        self.images.get(image_id)
    }

    pub fn get_max_num_layers(&self) -> usize {
        self.max_num_layers
    }
}
