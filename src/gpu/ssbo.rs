use std::ffi::c_void;

///Structs need to implement this to be able to be stored in a SSBO
pub trait ISSBO {
    fn data(&self) -> (isize, *const std::ffi::c_void) {
        let len = std::mem::size_of_val(self) as isize;
        (len, std::ptr::addr_of!(self) as *const c_void)
    }
}

///Implements ISSBO for all vectors
impl<T> ISSBO for Vec<T> {
    fn data(&self) -> (isize, *const std::ffi::c_void) {
        let len = (std::mem::size_of::<T>() * self.len()) as isize;
        (len, self.as_ptr() as *const c_void)
    }
}

///Abstracts an OpenGL Shader Storage Buffer Object
pub struct SSBO {
    pub id: u32,
}

impl SSBO {
    pub fn from<T: ISSBO>(binding: u32, obj: T) -> SSBO {
        let mut ssbo_id: u32 = 0;

        unsafe {
            gl::GenBuffers(1, &mut ssbo_id);

            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, ssbo_id);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, binding, ssbo_id);

            let (len, data) = obj.data();
            gl::BufferData(gl::SHADER_STORAGE_BUFFER, len, data, gl::STATIC_DRAW);

            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }
        return SSBO { id: ssbo_id };
    }

    pub fn update<T: ISSBO>(&self, obj: T) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.id);

            let (len, data) = obj.data();
            gl::NamedBufferSubData(self.id, 0, len, data);

            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }
    }
}
