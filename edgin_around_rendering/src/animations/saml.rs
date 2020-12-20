use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::skeleton;
use crate::utils::errors as err;

const DEFAULT_SCALE: f32 = 1.0;
const DEFAULT_ANGLE: f32 = 0.0;
const DEFAULT_POSITION: f32 = 0.0;

fn default_false() -> bool {
    false
}

#[derive(Serialize, Deserialize, Debug)]
struct SamlHoverArea {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct SamlInteraction {
    hover_area: SamlHoverArea,
}

#[derive(Serialize, Deserialize, Debug)]
struct SamlSource {
    id: String,
    name: String,
    size_x: usize,
    size_y: usize,
    pivot_x: f32,
    pivot_y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct SamlBonePose {
    position_x: Option<f32>,
    position_y: Option<f32>,
    scale_x: Option<f32>,
    scale_y: Option<f32>,
    angle: Option<f32>,
    source_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SamlMusclePose {
    key: String,
    position_x: Option<f32>,
    position_y: Option<f32>,
    scale_x: Option<f32>,
    scale_y: Option<f32>,
    angle: Option<f32>,
    source_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SamlBone {
    id: String,
    parent: Option<String>,
    pose: SamlBonePose,
}

#[derive(Serialize, Deserialize, Debug)]
struct SamlMuscle {
    bone_id: String,
    timeline: Vec<SamlMusclePose>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SamlSkeleton {
    id: String,
    scale: f32,
    bones: Vec<SamlBone>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SamlAnimation {
    /// ID of the animation
    id: String,

    /// ID of the associated skeleton
    skeleton_id: String,

    /// Tells if this animation should be looped or played only once.
    #[serde(default = "default_false")]
    is_looped: bool,

    /// Length of the animation in time.
    length: f32,

    /// Key point in time of the animation.
    keys: HashMap<String, f32>,

    /// Define how animation parameters change in time.
    muscles: Vec<SamlMuscle>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SamlSpec {
    interaction: SamlInteraction,
    sources: Vec<SamlSource>,
    skeletons: Vec<SamlSkeleton>,
    animations: Vec<SamlAnimation>,
}

struct SkeletonInfo {
    skeleton: SamlSkeleton,
    bone_ids: HashMap<String, usize>,
}

impl SkeletonInfo {
    fn new(skeleton: SamlSkeleton, bone_ids: HashMap<String, usize>) -> Self {
        Self { skeleton, bone_ids }
    }
}

pub struct Parser {
    source_ids: HashMap<String, usize>,
    skeletons: HashMap<String, SkeletonInfo>,

    interaction: SamlInteraction,
    sources: Vec<SamlSource>,
    animations: Vec<SamlAnimation>,
}

impl Parser {
    pub fn new(path: &std::path::Path) -> Self {
        let file = std::fs::File::open(path).expect(&format!("{}: {:?}", err::FILE_FAILED, path));
        let spec: SamlSpec = serde_yaml::from_reader(&file).expect(err::YAML_FAILED);

        let mut source_ids = HashMap::new();
        for (index, source) in spec.sources.iter().enumerate() {
            source_ids.insert(source.id.clone(), index);
        }

        let mut skeletons = HashMap::new();
        for skeleton in spec.skeletons {
            let bone_ids = Self::make_bone_ids(&skeleton);
            skeletons.insert(skeleton.id.clone(), SkeletonInfo::new(skeleton, bone_ids));
        }

        Self {
            source_ids: source_ids,
            skeletons: skeletons,
            interaction: spec.interaction,
            sources: spec.sources,
            animations: spec.animations,
        }
    }

    pub fn get_sources(&self) -> Vec<&str> {
        self.sources.iter().map(|source| source.name.as_str()).collect()
    }

    pub fn to_skeleton(&self) -> skeleton::Skeleton {
        let interaction = self.prepare_interaction();

        let mut images = Vec::with_capacity(self.sources.len());
        for source in self.sources.iter() {
            images.push(self.prepare_image(&source));
        }

        let mut animations = Vec::with_capacity(self.animations.len());
        for animation in self.animations.iter() {
            animations.push(self.prepare_animation(&animation));
        }

        skeleton::Skeleton::new(animations, images, interaction)
    }
}

impl Parser {
    fn make_bone_ids(skeleton: &SamlSkeleton) -> HashMap<String, usize> {
        let mut bone_ids = HashMap::new();
        for (index, bone) in skeleton.bones.iter().enumerate() {
            bone_ids.insert(bone.id.clone(), index);
        }
        bone_ids
    }

    fn prepare_interaction(&self) -> skeleton::Interaction {
        skeleton::Interaction::new(
            self.interaction.hover_area.left,
            self.interaction.hover_area.right,
            self.interaction.hover_area.top,
            self.interaction.hover_area.bottom,
        )
    }

    fn prepare_image(&self, source: &SamlSource) -> skeleton::Image {
        skeleton::Image::new(
            (source.pivot_x, source.pivot_y),
            (source.size_x as f32, source.size_y as f32),
        )
    }

    fn prepare_bone(
        &self,
        bone: &SamlBone,
        muscle: Option<&SamlMuscle>,
        keys: &HashMap<String, f32>,
        source_ids: &HashMap<String, usize>,
        bone_ids: &HashMap<String, usize>,
    ) -> skeleton::Bone {
        let parent_index = bone
            .parent
            .as_ref()
            .map(|id| bone_ids.get(id).expect(err::SAML_NOT_EXISTING_BONE))
            .cloned();

        let mut poses = Vec::new();
        if let Some(muscle) = muscle {
            for timeline_pose in &muscle.timeline {
                poses.push(self.prepare_muscle_pose(&bone.pose, &timeline_pose, keys, source_ids));
            }
        }

        if poses.len() == 0 {
            poses.push(self.prepare_bone_pose(&bone.pose, source_ids))
        }

        skeleton::Bone::new(parent_index, poses)
    }

    fn prepare_bone_pose(
        &self,
        pose: &SamlBonePose,
        source_ids: &HashMap<String, usize>,
    ) -> skeleton::Pose {
        let source_index = pose
            .source_id
            .as_ref()
            .map(|id| *source_ids.get(id).expect(err::SAML_NOT_EXISTING_SOURCE));
        let position = (
            pose.position_x.unwrap_or(DEFAULT_POSITION),
            pose.position_y.unwrap_or(DEFAULT_POSITION),
        );
        let scale = (pose.scale_x.unwrap_or(DEFAULT_SCALE), pose.scale_y.unwrap_or(DEFAULT_SCALE));
        let angle = 2.0 * std::f32::consts::PI * pose.angle.unwrap_or(DEFAULT_ANGLE);

        skeleton::Pose::new(0.0, source_index, position, scale, angle)
    }

    fn prepare_muscle_pose(
        &self,
        bone_pose: &SamlBonePose,
        muscle_pose: &SamlMusclePose,
        keys: &HashMap<String, f32>,
        source_ids: &HashMap<String, usize>,
    ) -> skeleton::Pose {
        let source_id = muscle_pose.source_id.as_ref().or(bone_pose.source_id.as_ref());
        let position_x =
            muscle_pose.position_x.unwrap_or(bone_pose.position_x.unwrap_or(DEFAULT_POSITION));
        let position_y =
            muscle_pose.position_y.unwrap_or(bone_pose.position_y.unwrap_or(DEFAULT_POSITION));
        let scale_x = muscle_pose.scale_x.unwrap_or(bone_pose.scale_x.unwrap_or(DEFAULT_SCALE));
        let scale_y = muscle_pose.scale_y.unwrap_or(bone_pose.scale_y.unwrap_or(DEFAULT_SCALE));
        let angle = muscle_pose.angle.unwrap_or(bone_pose.angle.unwrap_or(DEFAULT_ANGLE));
        let source_index =
            source_id.map(|id| *source_ids.get(id).expect(err::SAML_NOT_EXISTING_SOURCE));

        skeleton::Pose::new(
            *keys.get(&muscle_pose.key).expect(err::SAML_NOT_EXISTING_POSE),
            source_index,
            (position_x, position_y),
            (scale_x, scale_y),
            2.0 * std::f32::consts::PI * angle,
        )
    }

    fn prepare_animation(&self, animation: &SamlAnimation) -> skeleton::Animation {
        let mut bones = Vec::new();
        let info =
            self.skeletons.get(&animation.skeleton_id).expect(err::SAML_NOT_EXISTING_SKELETON);
        for bone in info.skeleton.bones.iter() {
            let muscle = self.find_muscle(&animation, &bone.id);
            bones.push(self.prepare_bone(
                bone,
                muscle,
                &animation.keys,
                &self.source_ids,
                &info.bone_ids,
            ));
        }

        skeleton::Animation::new(
            animation.id.clone(),
            animation.length,
            animation.is_looped,
            info.skeleton.scale,
            bones,
        )
    }

    fn find_muscle<'a>(
        &self,
        animation: &'a SamlAnimation,
        bone_id: &str,
    ) -> Option<&'a SamlMuscle> {
        animation.muscles.iter().find(|item| item.bone_id == bone_id)
    }
}
