use std::collections::HashSet;

use crate::utils::{
    errors as err,
    geometry::Matrix2D,
    ids::{MediumId, ResourceId},
    tile::Tile,
};

type BoneIndex = usize;

pub const DEFAULT_ANIMATION_NAME: &str = "idle";

#[derive(Debug)]
pub struct Interaction {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Interaction {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self { left, right, top, bottom }
    }
}

#[derive(Debug)]
pub struct Image {
    pivot: (f32, f32),
    size: (f32, f32),
}

impl Image {
    pub fn new(pivot: (f32, f32), size: (f32, f32)) -> Self {
        Self { pivot, size }
    }
}

#[derive(Clone, Debug)]
pub struct Pose {
    moment: f32,
    image_id: Option<MediumId>,
    position: (f32, f32),
    scale: (f32, f32),
    angle: f32,
}

impl Pose {
    pub fn new(
        moment: f32,
        image_id: Option<MediumId>,
        position: (f32, f32),
        scale: (f32, f32),
        angle: f32,
    ) -> Self {
        Self { moment, image_id, position, scale, angle }
    }

    fn calc_transformation(&self) -> Matrix2D {
        return Matrix2D::scale(self.scale)
            * Matrix2D::translation(self.position)
            * Matrix2D::rotation(self.angle);
    }
}

#[derive(Clone, Debug)]
pub struct Bone {
    parent_index: Option<BoneIndex>,
    poses: Vec<Pose>,
}

impl Bone {
    pub fn new(parent_index: Option<BoneIndex>, poses: Vec<Pose>) -> Self {
        Self { parent_index, poses }
    }

    pub fn calc_state_at(&self, moment: f32, duration: f32) -> (Matrix2D, Option<MediumId>) {
        if self.poses.len() < 2 {
            self.calc_state_at_single()
        } else {
            self.calc_state_at_multi(moment, duration)
        }
    }
}

impl Bone {
    fn calc_state_at_single(&self) -> (Matrix2D, Option<MediumId>) {
        let pose = self.poses.get(0).unwrap();
        (pose.calc_transformation(), pose.image_id)
    }

    fn calc_state_at_multi(&self, mut moment: f32, duration: f32) -> (Matrix2D, Option<MediumId>) {
        moment = moment % duration;
        let (pose1, pose2, moment1, moment2) =
            self.find_poses_for_moment(moment, duration).expect(err::SAML_NOT_EXISTING_POSE);

        let d = 1.0 / (moment2 - moment1);
        let w1 = d * (moment2 - moment);
        let w2 = d * (moment - moment1);

        let position = (
            w1 * pose1.position.0 + w2 * pose2.position.0,
            w1 * pose1.position.1 + w2 * pose2.position.1,
        );
        let scale =
            (w1 * pose1.scale.0 + w2 * pose2.scale.0, w1 * pose1.scale.1 + w2 * pose2.scale.1);
        let angle = w1 * pose1.angle + w2 * pose2.angle;

        let trans =
            Matrix2D::scale(scale) * Matrix2D::translation(position) * Matrix2D::rotation(angle);

        (trans, pose1.image_id)
    }

    fn find_poses_for_moment(
        &self,
        moment: f32,
        duration: f32,
    ) -> Option<(&Pose, &Pose, f32, f32)> {
        let last_pose = self.poses.last().unwrap();
        if moment < last_pose.moment {
            for i in 0..(self.poses.len() - 1) {
                let pose1 = &self.poses[i + 0];
                let pose2 = &self.poses[i + 1];
                if (pose1.moment <= moment) && (moment <= pose2.moment) {
                    return Some((pose1, pose2, pose1.moment, pose2.moment));
                }
            }
        } else {
            let pose1 = &last_pose;
            let pose2 = &self.poses[0];
            return Some((pose1, pose2, pose1.moment, duration));
        }
        None
    }
}

#[derive(Debug)]
pub struct Animation {
    name: String,
    duration: f32,
    is_looped: bool,
    scale: f32,
    bones: Vec<Bone>,
    draw_order: Vec<BoneIndex>,
}

impl Animation {
    pub fn new(
        name: String,
        duration: f32,
        is_looped: bool,
        scale: f32,
        unordered_bones: Vec<Bone>,
    ) -> Self {
        let mut calc_order = Vec::with_capacity(unordered_bones.len());
        let mut selected_ids = HashSet::<Option<BoneIndex>>::new();
        selected_ids.insert(None);
        while calc_order.len() < unordered_bones.len() {
            let mut new_ids = HashSet::<Option<BoneIndex>>::new();
            for (bone_index, bone) in unordered_bones.iter().enumerate() {
                if selected_ids.contains(&bone.parent_index) {
                    calc_order.push(bone_index);
                    new_ids.insert(Some(bone_index));
                }
            }
            selected_ids = new_ids;
        }

        let mut order_map = vec![0; unordered_bones.len()];
        for (new_index, &old_index) in calc_order.iter().enumerate() {
            *order_map.get_mut(old_index).unwrap() = new_index;
        }

        let mut bones = Vec::with_capacity(unordered_bones.len());
        for &old_index in calc_order.iter() {
            let mut bone = (*unordered_bones.get(old_index).unwrap()).clone();
            if let Some(parent_index) = bone.parent_index {
                bone.parent_index = order_map.get(parent_index).cloned();
            }
            bones.push(bone);
        }

        let mut draw_order = Vec::with_capacity(unordered_bones.len());
        for old_index in 0..unordered_bones.len() {
            draw_order.push(*order_map.get(old_index).unwrap());
        }

        Self { name, duration, is_looped, scale, bones, draw_order }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_duration(&self) -> f32 {
        self.duration
    }

    pub fn is_looped(&self) -> bool {
        self.is_looped
    }

    pub fn get_num_layers(&self) -> usize {
        self.bones.len()
    }

    pub fn tick(&self, moment: f32, skin_id: MediumId, images: &Vec<Image>) -> Vec<Tile> {
        // Prepare bone poses. Bones are sorted in such a way that parents are always prepared
        // before their children.
        // TODO: Avoid memory allocation.
        let mut info = Vec::<(Matrix2D, Option<MediumId>)>::with_capacity(self.bones.len());
        for bone in self.bones.iter() {
            let (mut trans, image_id) = bone.calc_state_at(moment, self.get_duration());
            if let Some(parent_index) = bone.parent_index {
                let (parent_trans, _) = info.get(parent_index).unwrap();
                trans = parent_trans * trans;
            }
            info.push((trans, image_id));
        }

        // Prepare tiles.
        let mut tiles = Vec::<Tile>::new();
        for &index in self.draw_order.iter() {
            let (trans, image_id) = info.get(index).unwrap();
            if let Some(image_id) = image_id {
                let resource_id = ResourceId::new(skin_id, *image_id);
                let scale = (self.scale, self.scale);
                let image = images.get(*image_id).expect(err::SAML_NOT_EXISTING_IMAGE);
                tiles.push(self.make_tile(resource_id, image, trans).scaled(scale));
            }
        }
        tiles
    }
}

impl Animation {
    fn make_tile(&self, resource_id: ResourceId, image: &Image, trans: &Matrix2D) -> Tile {
        let pivot = (-image.pivot.0, image.pivot.1 - image.size.1);
        let mut tile = Tile::new(resource_id, pivot, image.size);
        tile.transform(trans);
        tile
    }
}

pub struct Skeleton {
    selected_animation: String,
    animations: std::collections::HashMap<String, Animation>,
    images: Vec<Image>,
    interaction: Interaction,
}

impl Skeleton {
    pub fn new(
        mut animation_vector: Vec<Animation>,
        images: Vec<Image>,
        interaction: Interaction,
    ) -> Self {
        let selected_animation = DEFAULT_ANIMATION_NAME.to_string();
        let animations =
            animation_vector.drain(..).map(|anim| (anim.get_name().to_string(), anim)).collect();

        Self { selected_animation, animations, images, interaction }
    }

    pub fn get_interaction(&self) -> &Interaction {
        &self.interaction
    }

    pub fn get_selected_animation(&self) -> &Animation {
        self.animations
            .get(self.selected_animation.as_str())
            .expect(err::SAML_NOT_EXISTING_ANIMATION)
    }

    pub fn get_selected_animation_name(&self) -> &str {
        &self.selected_animation
    }

    pub fn select_animation(&mut self, name: &str) -> Result<(), ()> {
        if self.animations.contains_key(name) {
            self.selected_animation = name.to_string();
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn select_default_animation(&mut self) -> Result<(), ()> {
        self.select_animation(DEFAULT_ANIMATION_NAME)
    }

    pub fn get_animation_duration(&self) -> f32 {
        self.get_selected_animation().get_duration()
    }

    pub fn is_looped(&self) -> bool {
        self.get_selected_animation().is_looped()
    }

    pub fn get_max_num_layers(&self) -> usize {
        self.animations.values().map(|a| a.get_num_layers()).max().unwrap_or(0)
    }

    pub fn tick(&self, moment: f32, skin_id: MediumId) -> Vec<Tile> {
        self.get_selected_animation().tick(moment, skin_id, &self.images)
    }
}
