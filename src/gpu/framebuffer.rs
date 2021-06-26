///Abstracts an OpenGL framebuffer
pub struct Framebuffer {
    pub texture_id: u32,
    pub buffer_id: u32,
    pub res_x: i32,
    pub res_y: i32,
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.buffer_id);
            gl::DeleteTextures(1, &self.texture_id);
        }
    }
}

//Only one of these should exist
impl Framebuffer {
    ///Create an empty new frame buffer with a given resolution
    pub fn create(binding: u32, res_x: i32, res_y: i32) -> Framebuffer {
        let mut tex_id: u32 = 0;
        let mut fbuffer_id: u32 = 0;

        unsafe {
            gl::GenTextures(1, &mut tex_id);
            gl::ActiveTexture(gl::TEXTURE0 + binding);
            gl::BindTexture(gl::TEXTURE_2D, tex_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA32F as i32,
                res_x,
                res_y,
                0,
                gl::RGBA,
                gl::FLOAT,
                0 as *const std::ffi::c_void,
            );
            gl::BindImageTexture(
                binding,
                tex_id,
                0,
                gl::FALSE,
                0,
                gl::WRITE_ONLY,
                gl::RGBA32F,
            );

            gl::GenFramebuffers(1, &mut fbuffer_id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbuffer_id);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                tex_id,
                0,
            );
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, fbuffer_id);
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0);
        }

        return Framebuffer {
            texture_id: tex_id,
            buffer_id: fbuffer_id,
            res_x,
            res_y,
        };
    }

    ///Display the framebuffer
    pub fn blit(&self) {
        unsafe {
            gl::BlitFramebuffer(
                0,
                0,
                self.res_x,
                self.res_y,
                0,
                0,
                self.res_x,
                self.res_y,
                gl::COLOR_BUFFER_BIT,
                gl::LINEAR,
            );
        }
    }
}
