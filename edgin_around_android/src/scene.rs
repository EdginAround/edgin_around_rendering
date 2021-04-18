use jni::{
    objects::{JObject, ReleaseMode},
    sys::{jfloat, jobjectArray},
    JNIEnv,
};

use edgin_around_rendering::{
    game::{Actor, ElevationFunction, Scene},
    utils::{coordinates::Point, ids::ActorId},
};

use crate::{common, errors as err};

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_SceneBridge_initialize(
    env: JNIEnv,
    object: JObject,
) {
    let scene = edgin_around_rendering::game::Scene::new();
    common::set_holder(&env, &object, scene);
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_SceneBridge_configure(
    env: JNIEnv,
    object: JObject,
    hero_actor_id: common::ActorIdJni,
    elevation_object: JObject,
) {
    let mut scene = common::get_holder::<Scene>(&env, &object);
    let elevation = common::get_holder::<ElevationFunction>(&env, &elevation_object);
    scene.configure(hero_actor_id as ActorId, elevation.clone())
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_SceneBridge_getHeroId(
    env: JNIEnv,
    object: JObject,
) -> common::ActorIdJni {
    let scene = common::get_holder::<Scene>(&env, &object);
    scene.get_hero_id() as common::ActorIdJni
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_SceneBridge_createActors(
    env: JNIEnv,
    object: JObject,
    actors_array: jobjectArray,
) {
    let mut scene = common::get_holder::<Scene>(&env, &object);
    let mut actors_vec = Vec::<edgin_around_rendering::game::Actor>::new();
    for i in 0..env.get_array_length(actors_array).expect(err::JNI_ARRAY_LENGHT) {
        let element = env.get_object_array_element(actors_array, i).expect(err::JNI_ARRAY_ELEMENT);
        let actor = common::get_holder::<Actor>(&env, &element);
        actors_vec.push(actor.clone())
    }
    scene.create_actors(&actors_vec)
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_SceneBridge_deleteActors(
    env: JNIEnv,
    object: JObject,
    actor_ids_array: common::ActorIdArrayJni,
) {
    let mut scene = common::get_holder::<Scene>(&env, &object);
    let mut actor_ids_vec = Vec::<ActorId>::new();
    let actor_ids_length = env.get_array_length(actor_ids_array).expect(err::JNI_ARRAY_LENGHT);
    let actor_ids_data = env
        .get_array_elements::<common::ActorIdJni>(actor_ids_array, ReleaseMode::NoCopyBack)
        .expect(err::JNI_ARRAY_ELEMENTS);
    for i in 0..actor_ids_length as isize {
        actor_ids_vec.push(*actor_ids_data.as_ptr().offset(i) as ActorId)
    }
    scene.delete_actors(&actor_ids_vec)
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_SceneBridge_hideActors(
    env: JNIEnv,
    object: JObject,
    actor_ids_array: common::ActorIdArrayJni,
) {
    let mut scene = common::get_holder::<Scene>(&env, &object);
    let mut actor_ids_vec = Vec::new();
    let actor_ids_length = env.get_array_length(actor_ids_array).expect(err::JNI_ARRAY_ELEMENT);
    let actor_ids_data = env
        .get_array_elements::<common::ActorIdJni>(actor_ids_array, ReleaseMode::NoCopyBack)
        .expect(err::JNI_ARRAY_ELEMENTS);
    for i in 0..actor_ids_length as isize {
        actor_ids_vec.push(*actor_ids_data.as_ptr().offset(i) as ActorId)
    }
    scene.hide_actors(&actor_ids_vec)
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_SceneBridge_getRadius(
    env: JNIEnv,
    object: JObject,
) -> jfloat {
    let scene = common::get_holder::<Scene>(&env, &object);
    scene.get_radius()
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_SceneBridge_findClosestActors(
    env: JNIEnv,
    object: JObject,
    theta: jfloat,
    phi: jfloat,
    max_distance: jfloat,
) -> common::ActorIdArrayJni {
    let reference_point = Point::new(theta, phi);
    let scene = common::get_holder::<Scene>(&env, &object);
    let ids = scene
        .find_closest_actors(&reference_point, max_distance)
        .iter()
        .map(|id| *id as common::ActorIdJni)
        .collect::<Vec<common::ActorIdJni>>();
    let result = env.new_long_array(ids.len() as i32).expect(err::JNI_NEW_ARRAY);
    env.set_long_array_region(result, 0, ids.as_slice()).expect(err::JNI_ARRAY_REGION);
    result
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_SceneBridge_setActorPosition(
    env: JNIEnv,
    object: JObject,
    actor_id: common::ActorIdJni,
    theta: jfloat,
    phi: jfloat,
) {
    let mut scene = common::get_holder::<Scene>(&env, &object);
    if let Some(actor) = scene.get_actor_mut(actor_id as ActorId) {
        actor.set_position(Point::new(theta, phi));
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_SceneBridge_moveActorBy(
    env: JNIEnv,
    object: JObject,
    actor_id: common::ActorIdJni,
    distance: jfloat,
    bearing: jfloat,
) {
    let mut scene = common::get_holder::<Scene>(&env, &object);
    let radius = scene.get_radius();
    if let Some(actor) = scene.get_actor_mut(actor_id as ActorId) {
        actor.move_by(distance, bearing, radius);
    }
}
