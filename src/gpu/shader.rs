use gl::types::GLint;
use std::{ffi::CString, fs};

///Abstracts an OpenGL shader program
pub struct Shader {
    pub id: u32,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) }
    }
}

impl Shader {
    pub fn from(path: &str, shader_type: u32) -> Result<Shader, Box<dyn std::error::Error>> {
        let source = fs::read_to_string(path)?;

        let shader = Shader {
            id: unsafe { gl::CreateShader(shader_type) },
        };

        unsafe {
            let ptr: *const u8 = source.as_bytes().as_ptr();
            let ptr: *const i8 = std::mem::transmute(ptr);
            let len = source.len() as GLint;
            gl::ShaderSource(shader.id, 1, &ptr, &len);
        }

        let success = unsafe {
            gl::CompileShader(shader.id);

            let mut result: GLint = 0;
            gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut result);
            result != 0
        };

        if !success {
            return Err(shader.compilation_log())?;
        }

        let p_id = unsafe { gl::CreateProgram() };

        unsafe {
            gl::AttachShader(p_id, shader.id);
            gl::LinkProgram(p_id);

            gl::DeleteShader(shader.id);
        }

        let shader = Shader { id: p_id };

        return Ok(shader);
    }

    fn compilation_log(&self) -> String {
        let mut len = 0;
        unsafe { gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut len) };
        assert!(len > 0);

        let mut buf = Vec::with_capacity(len as usize);
        let buf_ptr = buf.as_mut_ptr() as *mut gl::types::GLchar;
        unsafe {
            gl::GetShaderInfoLog(self.id, len, std::ptr::null_mut(), buf_ptr);
            buf.set_len(len as usize);
        }

        match String::from_utf8(buf) {
            Ok(log) => log,
            Err(vec) => panic!("Couldn't convert compilation log: {}", vec),
        }
    }

    pub fn set_uint(&self, var: &str, value: u32) {
        unsafe {
            let s = CString::new(var).unwrap();
            let loc = gl::GetUniformLocation(self.id, s.as_ptr());
            gl::UseProgram(self.id);
            gl::Uniform1ui(loc, value);
        }
    }

    pub fn dispatch(&self, num_groups_x: u32, num_groups_y: u32, num_groups_z: u32, barrier: u32) {
        unsafe {
            gl::UseProgram(self.id);
            gl::DispatchCompute(num_groups_x, num_groups_y, num_groups_z);
            gl::MemoryBarrier(barrier);
        }
    }
}
