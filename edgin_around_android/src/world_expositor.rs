use jni::{
    objects::{JObject, JString, ReleaseMode},
    sys::{jfloat, jint, jobjectArray},
    JNIEnv,
};

use edgin_around_rendering::{
    expositors::WorldExpositor,
    game::{Actor, Scene},
    utils::ids::ActorId,
};

use crate::{common, errors as err};

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_WorldExpositorBridge_initialize(
    env: JNIEnv,
    object: JObject,
    resource_dir: JString,
    width: jint,
    height: jint,
) {
    common::initialize_once();

    let resource_dir = common::make_string(&env, resource_dir).expect(err::JNI_MAKE_STRING);
    let resource_path = std::path::PathBuf::from(&resource_dir);
    let world = WorldExpositor::new(resource_path, (width as usize, height as usize));
    common::set_holder(&env, &object, world);
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_WorldExpositorBridge_resize(
    env: JNIEnv,
    object: JObject,
    width: jint,
    height: jint,
) {
    let mut world = common::get_holder::<WorldExpositor>(&env, &object);
    world.resize(width as usize, height as usize)
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_WorldExpositorBridge_render(
    env: JNIEnv,
    object: JObject,
    scene_object: JObject,
) {
    let mut world = common::get_holder::<WorldExpositor>(&env, &object);
    let scene = common::get_holder::<Scene>(&env, &scene_object);
    world.render(&scene)
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_WorldExpositorBridge_getBearing(
    env: JNIEnv,
    object: JObject,
) -> jfloat {
    let world = common::get_holder::<WorldExpositor>(&env, &object);
    world.get_bearing()
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_WorldExpositorBridge_getHighlightedActorId(
    env: JNIEnv,
    object: JObject,
) -> common::ActorIdJni {
    let world = common::get_holder::<WorldExpositor>(&env, &object);
    // TODO: Return proper invalid value.
    world.get_highlighted_actor_id().unwrap_or(0) as common::ActorIdJni
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_WorldExpositorBridge_setHighlightedActorId(
    env: JNIEnv,
    object: JObject,
    actor_id: common::ActorIdJni,
) {
    let mut world = common::get_holder::<WorldExpositor>(&env, &object);
    world.set_highlighted_actor_id(Some(actor_id as ActorId))
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_WorldExpositorBridge_removeHighlight(
    env: JNIEnv,
    object: JObject,
) {
    let mut world = common::get_holder::<WorldExpositor>(&env, &object);
    world.set_highlighted_actor_id(None)
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_WorldExpositorBridge_zoomBy(
    env: JNIEnv,
    object: JObject,
    zoom: jfloat,
) {
    let mut world = common::get_holder::<WorldExpositor>(&env, &object);
    world.zoom_by(zoom)
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_WorldExpositorBridge_rotateBy(
    env: JNIEnv,
    object: JObject,
    angle: jfloat,
) {
    let mut world = common::get_holder::<WorldExpositor>(&env, &object);
    world.rotate_by(angle)
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_WorldExpositorBridge_tiltBy(
    env: JNIEnv,
    object: JObject,
    angle: jfloat,
) {
    let mut world = common::get_holder::<WorldExpositor>(&env, &object);
    world.tilt_by(angle)
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_WorldExpositorBridge_createRenderers(
    env: JNIEnv,
    object: JObject,
    actors_array: jobjectArray,
) {
    let mut world = common::get_holder::<WorldExpositor>(&env, &object);
    let mut actors_vec = Vec::<Actor>::new();
    for i in 0..env.get_array_length(actors_array).expect(err::JNI_ARRAY_LENGHT) {
        let element = env.get_object_array_element(actors_array, i).expect(err::JNI_ARRAY_ELEMENT);
        let actor = common::get_holder::<Actor>(&env, &element);
        actors_vec.push(actor.clone())
    }
    world.create_renderers(&actors_vec)
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_WorldExpositorBridge_deleteRenderers(
    env: JNIEnv,
    object: JObject,
    actor_ids_array: common::ActorIdArrayJni,
) {
    let mut world = common::get_holder::<WorldExpositor>(&env, &object);
    let mut actor_ids_vec = Vec::<ActorId>::new();
    let actor_ids_length =
        env.get_array_length(actor_ids_array).expect(err::JNI_ARRAY_LENGHT) as isize;
    let actor_ids_data = env
        .get_array_elements::<common::ActorIdJni>(actor_ids_array, ReleaseMode::NoCopyBack)
        .expect(err::JNI_ARRAY_ELEMENTS);
    for i in 0..actor_ids_length {
        actor_ids_vec.push(*actor_ids_data.as_ptr().offset(i) as ActorId)
    }
    world.delete_renderers(&actor_ids_vec)
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_WorldExpositorBridge_selectVariant(
    env: JNIEnv,
    object: JObject,
    actor_id: common::ActorIdJni,
    variant_name: JString,
) {
    let mut world = common::get_holder::<WorldExpositor>(&env, &object);
    let variant_name = common::make_string(&env, variant_name).expect(err::JNI_MAKE_STRING);
    world.play_animation(actor_id as ActorId, &variant_name)
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_WorldExpositorBridge_playAnimation(
    env: JNIEnv,
    object: JObject,
    actor_id: common::ActorIdJni,
    animation_name: JString,
) {
    let mut world = common::get_holder::<WorldExpositor>(&env, &object);
    let animation_name = common::make_string(&env, animation_name).expect(err::JNI_MAKE_STRING);
    world.play_animation(actor_id as ActorId, &animation_name)
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_WorldExpositorBridge_attachActor(
    env: JNIEnv,
    object: JObject,
    hook_name: JString,
    base_actor_id: common::ActorIdJni,
    attached_actor_id: common::ActorIdJni,
) {
    let hook_name = common::make_string(&env, hook_name).expect(err::JNI_MAKE_STRING);
    let mut world = common::get_holder::<WorldExpositor>(&env, &object);
    world.attach_actor(hook_name, base_actor_id as ActorId, Some(attached_actor_id as ActorId))
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_WorldExpositorBridge_detachActor(
    env: JNIEnv,
    object: JObject,
    hook_name: JString,
    base_actor_id: common::ActorIdJni,
) {
    let hook_name = common::make_string(&env, hook_name).expect(err::JNI_MAKE_STRING);
    let mut world = common::get_holder::<WorldExpositor>(&env, &object);
    world.attach_actor(hook_name, base_actor_id as ActorId, None)
}
