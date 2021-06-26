use std::ffi::c_void;

pub fn init() {
    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(Some(gl_callback), 0 as *const c_void);
    }
}

extern "system" fn gl_callback(
    source: u32,
    gltype: u32,
    _: u32,
    severity: u32,
    len: i32,
    message: *const i8,
    _: *mut c_void,
) {
    unsafe {
        println!(
            "GL: source = {}, type = {}, severity = {}, message = {:?}",
            gl_sources_to_string(source),
            gl_message_to_string(gltype),
            gl_error_to_string(severity),
            String::from_raw_parts(message as *mut u8, len as usize, len as usize),
        );
    }
}

fn gl_error_to_string(err: u32) -> &'static str {
    match err {
        gl::DEBUG_SEVERITY_HIGH => "HIGH",
        gl::DEBUG_SEVERITY_MEDIUM => "MEDIUM",
        gl::DEBUG_SEVERITY_LOW => "LOW",
        _ => "UNKOWN",
    }
}

fn gl_message_to_string(msg: u32) -> &'static str {
    match msg {
        gl::DEBUG_TYPE_ERROR => "ERROR",
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "DEPRECATED_BEHAVIOUR",
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "UNDEFINED_BEHAVIOUR",
        gl::DEBUG_TYPE_PORTABILITY => "PORTABILITY",
        gl::DEBUG_TYPE_PERFORMANCE => "PERFORMANCE",
        gl::DEBUG_TYPE_MARKER => "MARKER",
        gl::DEBUG_TYPE_PUSH_GROUP => "PUSH_GROUP",
        gl::DEBUG_TYPE_POP_GROUP => "POP_GROUP",
        gl::DEBUG_TYPE_OTHER => "OTHER",
        _ => "UNKNOWN",
    }
}

fn gl_sources_to_string(src: u32) -> &'static str {
    match src {
        gl::DEBUG_SOURCE_API => "API",
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => "WINDOW_SYSTEM",
        gl::DEBUG_SOURCE_SHADER_COMPILER => "SHADER_COMPILER",
        gl::DEBUG_SOURCE_THIRD_PARTY => "THIRD_PARTY",
        gl::DEBUG_SOURCE_APPLICATION => "APPLICATION",
        gl::DEBUG_SOURCE_OTHER => "OTHER",
        _ => "UNKNOWN",
    }
}
