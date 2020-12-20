pub type ActorId = usize;
pub type MediumId = usize;
pub type TextureId = u32;
pub type ResourcePath = (String, String);

pub const NO_TEXTURE: TextureId = 0;

#[derive(Clone, Debug)]
pub struct ResourceId {
    pub skin_id: MediumId,
    pub image_id: MediumId,
}

impl ResourceId {
    pub fn new(skin_id: MediumId, image_id: MediumId) -> Self {
        Self { skin_id, image_id }
    }
}
