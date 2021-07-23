use std::ffi::c_void;

use super::spriteasset::SpriteAsset;

pub struct Tilesheet {
    pixels: Vec<u8>,

    tile_width: i32,
    tile_height: i32,
    tile_count: i32,

    columns: i32,
    rows: i32,
}

pub trait VecTilesheetExt {
    fn max_width(&self) -> i32;
    fn max_height(&self) -> i32;
    fn total_tile_count(&self) -> i32;
}

impl VecTilesheetExt for Vec<Tilesheet> {
    fn max_width(&self) -> i32 {
        let mut value = i32::MIN;
        self.iter().for_each(|tilesheet| {
            value = value.max(tilesheet.tile_width);
        });
        value
    }

    fn max_height(&self) -> i32 {
        let mut value = i32::MIN;
        self.iter().for_each(|tilesheet| {
            value = value.max(tilesheet.tile_height);
        });
        value
    }

    fn total_tile_count(&self) -> i32 {
        let mut sum = 0 as i32;
        self.iter().for_each(|tilesheet| {
            sum += tilesheet.tile_count;
        });
        sum
    }
}

impl Tilesheet {
    pub fn from(asset: &SpriteAsset) -> Result<Tilesheet, Box<dyn std::error::Error>> {
        let img_file = image::open(&asset.file)?;
        let img_file = img_file.as_rgba8().unwrap();

        let tile_width = img_file.width() / asset.columns;
        let tile_height = img_file.height() / asset.rows;

        let mut pixels =
            Vec::with_capacity((tile_width * tile_width * 4 * asset.columns * asset.rows) as usize);

        for row in 0..asset.rows {
            for column in 0..asset.columns {
                for y in (0..tile_height).rev() {
                    for x in 0..tile_width {
                        let pixel = img_file
                            .get_pixel(column * tile_width + x, row * tile_height + y)
                            .0;

                        pixels.push(pixel[0]);
                        pixels.push(pixel[1]);
                        pixels.push(pixel[2]);
                        pixels.push(pixel[3]);
                    }
                }
            }
        }

        let tile_width = tile_width as i32;
        let tile_height = tile_height as i32;
        let tile_count = (asset.columns * asset.rows) as i32;

        Ok(Tilesheet {
            pixels,
            tile_width,
            tile_height,
            tile_count,
            columns: asset.columns as i32,
            rows: asset.rows as i32,
        })
    }

    pub fn tile(&self, index: i32) -> *const c_void {
        let pixel_per_tile = self.tile_width * self.tile_height * 4;
        let index = index * pixel_per_tile;
        &self.pixels[index as usize] as *const _ as *const c_void
    }

    pub fn tile_width(&self) -> i32 {
        self.tile_width
    }

    pub fn tile_height(&self) -> i32 {
        self.tile_height
    }

    pub fn tile_count(&self) -> i32 {
        self.tile_count
    }

    pub fn columns(&self) -> i32 {
        self.columns
    }

    pub fn rows(&self) -> i32 {
        self.rows
    }
}
