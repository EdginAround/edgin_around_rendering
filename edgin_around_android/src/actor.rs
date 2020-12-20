use jni::{
    objects::{JObject, JString},
    JNIEnv,
};

use edgin_around_rendering::{utils::coordinates::Point, utils::ids::ActorId};

use crate::{common, errors as err};

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_ActorBridge_initialize(
    env: JNIEnv,
    object: JObject,
    id: common::ActorIdJni,
    entity_name: JString,
    point_object: JObject,
) {
    let entity_name = common::make_string(&env, entity_name).expect(err::JNI_MAKE_STRING);
    let point = if *point_object != std::ptr::null_mut() {
        Some(common::get_holder::<Point>(&env, &point_object).clone())
    } else {
        None
    };
    let actor = edgin_around_rendering::game::Actor::new(id as ActorId, entity_name, point);
    common::set_holder(&env, &object, actor);
}
