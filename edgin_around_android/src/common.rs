use jni::{
    objects::{JObject, JString},
    sys::{jlong, jlongArray},
    JNIEnv,
};

use crate::errors as err;

pub type ActorIdJni = jlong;
pub type ActorIdArrayJni = jlongArray;

pub const HOLDER_FIELD_NAME: &str = "nativePtrHolder";

pub static INIT: std::sync::Once = std::sync::Once::new();

pub fn make_string(env: &JNIEnv, recipient: JString) -> Option<String> {
    env.get_string(recipient).ok().map(|s| s.into())
}

pub fn initialize_once() {
    INIT.call_once(|| {
        // Configure logger
        let conf = android_logger::Config::default()
            .with_min_level(log::Level::Info)
            .with_tag("EdginAround");
        android_logger::init_once(conf);

        // Set panic hook
        std::panic::set_hook(Box::new(|msg| {
            log::error!("EdginAround: {:?}", msg);
        }));

        // Init the library
        edgin_around_rendering::init().expect("Initialize EdginAround");
    });
}

pub fn set_holder<T>(env: &JNIEnv, object: &JObject, value: T)
where
    T: Send + 'static,
{
    env.set_rust_field(*object, HOLDER_FIELD_NAME, value).expect(err::JNI_SET_HANDLER)
}

pub fn get_holder<'a, T>(env: &'a JNIEnv, object: &'a JObject) -> std::sync::MutexGuard<'a, T>
where
    T: Send + 'static,
{
    env.get_rust_field::<JObject, &str, T>(*object, HOLDER_FIELD_NAME).expect(err::JNI_GET_HANDLER)
}
