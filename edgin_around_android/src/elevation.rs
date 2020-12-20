use jni::{
    objects::{JObject, JString},
    sys::jfloat,
    JNIEnv,
};

use edgin_around_rendering::game::ElevationFunction;

use crate::{common, errors as err};

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_ElevationBridge_initialize(
    env: JNIEnv,
    object: JObject,
    radius: jfloat,
) {
    let elevation = ElevationFunction::new(radius);
    common::set_holder(&env, &object, elevation);
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_ElevationBridge_addTerrain(
    env: JNIEnv,
    object: JObject,
    name: JString,
    theta: jfloat,
    phi: jfloat,
) {
    let mut elevation = common::get_holder::<ElevationFunction>(&env, &object);
    let name = common::make_string(&env, name).expect(err::JNI_MAKE_STRING);
    elevation.add_terrain(&name, theta, phi)
}
