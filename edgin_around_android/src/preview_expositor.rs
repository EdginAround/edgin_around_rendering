use jni::{
    objects::{JObject, JString},
    sys::jint,
    JNIEnv,
};

use edgin_around_rendering::expositors::PreviewExpositor;

use crate::{common, errors as err};

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_PreviewExpositorBridge_initialize(
    env: JNIEnv,
    object: JObject,
    sprite_dir: JString,
    skin_name: JString,
    saml_name: JString,
    variant_name: JString,
    action_name: JString,
    width: jint,
    height: jint,
) {
    common::initialize_once();

    let sprite_dir = common::make_string(&env, sprite_dir).expect(err::JNI_MAKE_STRING);
    let skin_name = common::make_string(&env, skin_name).expect(err::JNI_MAKE_STRING);
    let saml_name = common::make_string(&env, saml_name).expect(err::JNI_MAKE_STRING);
    let variant_name = common::make_string(&env, variant_name).expect(err::JNI_MAKE_STRING);
    let action_name = common::make_string(&env, action_name).expect(err::JNI_MAKE_STRING);
    let sprite_path = std::path::Path::new(&sprite_dir);

    let preview = PreviewExpositor::new(
        sprite_path,
        &skin_name,
        &saml_name,
        &variant_name,
        &action_name,
        (width as usize, height as usize),
    );
    common::set_holder(&env, &object, preview);
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_PreviewExpositorBridge_resize(
    env: JNIEnv,
    object: JObject,
    width: jint,
    height: jint,
) {
    let mut preview = common::get_holder::<PreviewExpositor>(&env, &object);
    preview.resize(width as usize, height as usize);
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_PreviewExpositorBridge_render(
    env: JNIEnv,
    object: JObject,
) {
    let mut preview = common::get_holder::<PreviewExpositor>(&env, &object);
    preview.render();
}
