use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
};

use crate::utils::{
    errors as err,
    ids::{MediumId, ResourceId, ResourcePath, TextureId, NO_TEXTURE},
};

const TILES_DIR: &str = "tiles";
const SPRITES_DIR: &str = "sprites";
const GRASS_FILE: &str = "grass.png";
const WATER_FILE: &str = "water.png";

fn load_image(path: &Path) -> TextureId {
    let file = File::open(path).expect(&format!("{}: {:?}", err::FILE_FAILED, path));
    let decoder = png::Decoder::new(file);
    let (info, mut reader) = decoder.read_info().expect(err::PNG_FAILED);
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).expect(err::PNG_FAILED);

    let mut texture_id = 0;
    unsafe {
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as gl::types::GLint);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as gl::types::GLint,
            info.width as gl::types::GLint,
            info.height as gl::types::GLint,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            buf.as_ptr() as *const _,
        );
    }
    texture_id
}

pub fn sprites_path(base: &Path) -> PathBuf {
    base.join(SPRITES_DIR)
}

pub struct Textures {
    pub water: TextureId,
    pub grass: TextureId,
}

impl Textures {
    pub fn load(resource_dir: &Path) -> Self {
        let water_path = resource_dir.join(TILES_DIR).join(WATER_FILE);
        let grass_path = resource_dir.join(TILES_DIR).join(GRASS_FILE);

        Self { water: load_image(water_path.as_path()), grass: load_image(grass_path.as_path()) }
    }
}

impl Default for Textures {
    fn default() -> Self {
        Self { water: NO_TEXTURE, grass: NO_TEXTURE }
    }
}

pub struct Sprites {
    sprites_dir: PathBuf,
    id_map: HashMap<ResourcePath, ResourceId>,
    skins: Vec<Vec<TextureId>>,
    loaded_skins: HashMap<String, MediumId>,
}

impl Sprites {
    pub fn new(sprites_dir: PathBuf) -> Self {
        Self {
            sprites_dir: sprites_dir,
            id_map: HashMap::new(),
            skins: Vec::new(),
            loaded_skins: HashMap::new(),
        }
    }

    pub fn get_sprites_dir(&self) -> &Path {
        self.sprites_dir.as_path()
    }

    pub fn get_resource_id(&self, key: &(String, String)) -> Option<ResourceId> {
        self.id_map.get(key).cloned()
    }

    pub fn get_texture_id(&self, resource_id: &ResourceId) -> Option<TextureId> {
        if let Some(skin) = &self.skins.get(resource_id.skin_id) {
            skin.get(resource_id.image_id).cloned()
        } else {
            None
        }
    }

    pub fn load_skin(&mut self, skin_name: &str, image_names: &Vec<&str>) -> MediumId {
        let skin_dir = Path::new(skin_name);
        let skin_path = self.sprites_dir.join(skin_dir);
        let skin_id = self.skins.len();
        let mut skin = Vec::new();
        for (image_id, &image_name) in image_names.iter().enumerate() {
            let image_path = skin_path.join(image_name).with_extension("png");
            if image_path.is_file() {
                let texture_id = load_image(&image_path);
                let resource_id = ResourceId::new(skin_id, image_id);
                let key = (skin_name.to_string(), image_name.to_string());
                skin.push(texture_id);
                self.id_map.insert(key, resource_id);
            }
        }
        let skin_id = self.skins.len();
        self.skins.push(skin);
        self.loaded_skins.insert(skin_name.to_string(), skin_id);
        skin_id
    }

    pub fn load_skin_if_needed(&mut self, skin_name: &str, image_names: &Vec<&str>) -> MediumId {
        if let Some(skin_id) = self.loaded_skins.get(skin_name) {
            *skin_id
        } else {
            self.load_skin(skin_name, image_names)
        }
    }
}
