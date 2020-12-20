use gl;

const GROUND_VERTEX: &str = include_str!("../../shaders/ground_vertex.glsl");
const GROUND_FRAGMENT: &str = include_str!("../../shaders/ground_fragment.glsl");
const ENTITIES_VERTEX: &str = include_str!("../../shaders/entities_vertex.glsl");
const ENTITIES_FRAGMENT: &str = include_str!("../../shaders/entities_fragment.glsl");

/// Initializes OpenGL library.
pub fn init() -> Result<(), ()> {
    gl::load_with(|s| egl::get_proc_address(s) as *const std::os::raw::c_void);
    Ok(())
}

/// Get GL info log.
pub fn get_info_log(object: gl::types::GLuint) -> String {
    unsafe {
        let mut log_length: i32 = 0;
        if gl::IsShader(object) == gl::TRUE {
            gl::GetShaderiv(object, gl::INFO_LOG_LENGTH, &mut log_length);
        } else if gl::IsProgram(object) == gl::TRUE {
            gl::GetProgramiv(object, gl::INFO_LOG_LENGTH, &mut log_length);
        } else {
            return "GL: Not a shader or a program".to_owned();
        }

        let mut length: i32 = 0;
        let mut buffer = vec![0u8; log_length as usize];
        if gl::IsShader(object) == gl::TRUE {
            gl::GetShaderInfoLog(
                object,
                buffer.len() as i32,
                &mut length,
                buffer.as_mut_ptr() as *mut gl::types::GLbyte,
            );
        } else if gl::IsProgram(object) == gl::TRUE {
            gl::GetProgramInfoLog(
                object,
                buffer.len() as i32,
                &mut length,
                buffer.as_mut_ptr() as *mut gl::types::GLbyte,
            );
        }

        let cstr = std::ffi::CStr::from_ptr(std::mem::transmute(&buffer));
        format!("GL: {}", std::str::from_utf8(cstr.to_bytes()).expect("Info log is invalid"))
    }
}

/// Create and compile shader.
pub fn create_shader(
    source: &str,
    shader_type: gl::types::GLenum,
) -> Result<gl::types::GLuint, String> {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let cstr = std::ffi::CString::new(source.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &cstr.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        let mut status = gl::FALSE as gl::types::GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        if status != (gl::TRUE as gl::types::GLint) {
            let info_log = get_info_log(shader);
            gl::DeleteShader(shader);
            Err(info_log)
        } else {
            Ok(shader)
        }
    }
}

/// Create and link shader program.
pub fn create_program(
    vertex_shader: gl::types::GLenum,
    fragment_shader: gl::types::GLenum,
) -> Result<gl::types::GLuint, String> {
    unsafe {
        // Create program
        let shader_program = gl::CreateProgram();

        // Link with shaders
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // Handle errors
        let mut link_ok = gl::FALSE as i32;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut link_ok);
        if link_ok == gl::TRUE as i32 {
            Ok(shader_program)
        } else {
            let info_log = get_info_log(shader_program);
            gl::DeleteProgram(shader_program);
            Err(info_log)
        }
    }
}

/// Create program and link with shaders.
pub fn prepare_shader_program(
    vertex_source: &str,
    fragment_source: &str,
) -> Result<gl::types::GLuint, String> {
    // Create vertex shader
    let vertex_shader = create_shader(vertex_source, gl::VERTEX_SHADER)?;

    // Create fragment shader
    let fragment_shader = create_shader(fragment_source, gl::FRAGMENT_SHADER)?;

    // Create and link shader program
    create_program(vertex_shader, fragment_shader)
}

/// Prepares shader for rendering ground and water.
pub fn prepare_ground_shader_program() -> Result<gl::types::GLuint, String> {
    prepare_shader_program(GROUND_VERTEX, GROUND_FRAGMENT)
}

/// Prepares shader for rendering entities.
pub fn prepare_entities_shader_program() -> Result<gl::types::GLuint, String> {
    prepare_shader_program(ENTITIES_VERTEX, ENTITIES_FRAGMENT)
}

/// Get location attribute variable in linked program.
pub fn get_attrib_location(
    program: gl::types::GLuint,
    name: String,
) -> Result<gl::types::GLint, String> {
    let cstr = std::ffi::CString::new(name.as_bytes()).unwrap();
    let location = unsafe {
        gl::GetAttribLocation(
            program,
            cstr.as_bytes_with_nul().as_ptr() as *const gl::types::GLbyte,
        )
    };

    if location < 0 {
        Err(format!("Could not get location for attribute '{}'", name))
    } else {
        Ok(location)
    }
}

/// Get location of uniform variable in linked program.
pub fn get_uniform_location(
    program: gl::types::GLuint,
    name: String,
) -> Result<gl::types::GLint, String> {
    let cstr = std::ffi::CString::new(name.as_bytes()).unwrap();
    let location = unsafe {
        gl::GetUniformLocation(
            program,
            cstr.as_bytes_with_nul().as_ptr() as *const gl::types::GLbyte,
        )
    };

    if location < 0 {
        Err(format!("Could not get location for uniform '{}'", name))
    } else {
        Ok(location)
    }
}
