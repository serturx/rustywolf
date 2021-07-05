use std::ffi::c_void;

pub struct TextureSampler {
    pub id: u32,
}

impl Drop for TextureSampler {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id) }
    }
}

impl TextureSampler {
    pub fn from(binding: u32, img: &image::DynamicImage, tile_width: u32) -> TextureSampler {
        let mut tex_id: u32 = 0;

        let b = img.as_rgba8().unwrap();

        let (width, height) = b.dimensions();
        let (rows, columns) = (height / tile_width, width / tile_width);

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
                tile_width as i32,
                tile_width as i32,
                (rows * columns) as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                std::ptr::null() as *const c_void,
            );
        }

        let mut tile_pixels = vec![0; (tile_width * tile_width * 4) as usize];
        let mut tile_idx: i32 = 0;

        for row in 0..rows {
            for column in 0..columns {
                let mut pixel_idx = 0;
                for y in (0..tile_width).rev() {
                    for x in 0..tile_width {
                        let pixel = b.get_pixel(column * tile_width + x, row * tile_width + y).0;

                        tile_pixels[pixel_idx + 0] = pixel[0];
                        tile_pixels[pixel_idx + 1] = pixel[1];
                        tile_pixels[pixel_idx + 2] = pixel[2];
                        tile_pixels[pixel_idx + 3] = pixel[3];

                        pixel_idx += 4;
                    }
                }

                unsafe {
                    gl::TexSubImage3D(
                        gl::TEXTURE_2D_ARRAY,
                        0,
                        0,
                        0,
                        tile_idx,
                        tile_width as i32,
                        tile_width as i32,
                        1,
                        gl::RGBA,
                        gl::UNSIGNED_BYTE,
                        tile_pixels.as_mut_ptr() as *const c_void,
                    );
                }
                tile_idx += 1;
            }
        }

        unsafe {
            gl::GenerateMipmap(gl::TEXTURE_2D_ARRAY);
        }

        return TextureSampler { id: tex_id };
    }
}
