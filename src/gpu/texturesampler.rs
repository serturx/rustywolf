use std::ffi::c_void;

pub struct TextureSampler {
    pub id: u32,
    pub binding: u32,
}

impl Drop for TextureSampler {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id) }
    }
}

impl TextureSampler {
    pub fn from(
        binding: u32,
        width: i32,
        height: i32,
        depth: i32,
        data: *const c_void,
    ) -> TextureSampler {
        let mut tex_id: u32 = 0;

        unsafe {
            gl::GenTextures(1, &mut tex_id);

            gl::ActiveTexture(gl::TEXTURE0 + binding);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, tex_id);

            gl::TexParameteri(
                gl::TEXTURE_2D_ARRAY,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D_ARRAY,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as i32,
            );

            gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            gl::TexImage3D(
                gl::TEXTURE_2D_ARRAY,
                0,
                gl::RGBA8 as i32,
                width,
                height,
                depth,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D_ARRAY);
        }

        TextureSampler {
            id: tex_id,
            binding,
        }
    }

    pub fn update(
        &self,
        xoffset: i32,
        yoffset: i32,
        zoffset: i32,
        width: i32,
        height: i32,
        depth: i32,
        data: *const c_void,
    ) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + self.binding);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, self.id);

            gl::TexSubImage3D(
                gl::TEXTURE_2D_ARRAY,
                0,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D_ARRAY);
        }
    }
}
