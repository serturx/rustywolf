use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
#[repr(C)]
struct WorldStructure {
    layout_tile_width: u32,
    layout_stride: u32,
    layout: Vec<u32>,

    ceiling_idx: u32,
    floor_idx: u32,
}

impl WorldStructure {
    fn as_vec_for_gpu(&self) -> Vec<u32> {
        let mut data: Vec<u32> = Vec::new();

        data.push(self.floor_idx);
        data.push(self.ceiling_idx);
        data.push(self.layout_tile_width);
        data.push(self.layout_stride);

        for tile in &self.layout {
            data.push(*tile);
        }

        return data;
    }
}

const RES_MAPS: &str = "./res/maps/";

pub struct World {
    identifier: String,
    structure: WorldStructure,
    spritesheet: image::DynamicImage,
}

impl World {
    pub fn load(identifier: &str) -> Result<World, Box<dyn std::error::Error>> {
        let full_path = RES_MAPS.to_owned() + identifier + "/";

        let layout_path = String::from(full_path.clone() + "layout.ron");
        let layout_file_content = fs::read_to_string(layout_path)?;
        let layout: WorldStructure = ron::from_str(&layout_file_content)?;

        let spritesheet_path = full_path + "sheet.png";
        let spritesheet = image::open(spritesheet_path)?;

        let world = World {
            identifier: String::from(identifier),
            structure: layout,
            spritesheet,
        };

        Ok(world)
    }

    pub fn as_vec_for_gpu(&self) -> Vec<u32> {
        return self.structure.as_vec_for_gpu();
    }

    pub fn at(&self, x: u32, y: u32) -> &u32 {
        return &self.structure.layout[(x + y * self.structure.layout_stride) as usize];
    }

    pub fn sampler_data(&self) -> (&image::DynamicImage, u32) {
        return (&self.spritesheet, self.structure.layout_tile_width);
    }
}
