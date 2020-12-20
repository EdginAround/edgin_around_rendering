use jni::{objects::JObject, sys::jobjectArray, JNIEnv};

use crate::errors as err;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_edgin_around_rendering_AboutBridge_getVersion(
    env: JNIEnv,
    _object: JObject,
) -> jobjectArray {
    let string_class = env.find_class("java/lang/String").expect(err::JNI_FIND_CLASS);
    let initial_element = env.new_string("").expect(err::JNI_NEW_STRING);
    let version = edgin_around_rendering::get_version();
    let result = env
        .new_object_array(version.len() as i32, string_class, *initial_element)
        .expect(err::JNI_NEW_ARRAY);
    for (i, subversion) in version.iter().enumerate() {
        let new_element = env.new_string(subversion).expect(err::JNI_NEW_STRING);
        env.set_object_array_element(result, i as i32, *new_element).expect(err::JNI_ARRAY_ELEMENT);
    }
    result
}
