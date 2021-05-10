use jni::{objects::JObject, sys::jfloat, JNIEnv};

use edgin_around_rendering::utils::coordinates::Point;

use crate::common;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_PointBridge_initialize(
    env: JNIEnv,
    object: JObject,
    theta: jfloat,
    phi: jfloat,
) {
    let point = Point::new(theta, phi);
    common::set_holder(&env, &object, point);
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_PointBridge_getTheta(
    env: JNIEnv,
    object: JObject,
) -> jfloat {
    let point = common::get_holder::<Point>(&env, &object);
    point.theta
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_PointBridge_getPhi(
    env: JNIEnv,
    object: JObject,
) -> jfloat {
    let point = common::get_holder::<Point>(&env, &object);
    point.phi
}
