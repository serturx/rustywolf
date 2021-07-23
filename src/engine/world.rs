use serde::Deserialize;
use std::{ffi::OsString, fs};

use crate::gpu::{self, Framebuffer, Shader, SSBO};

use super::{
    sprites::{SpriteAsset, SpriteRegistry, Tilesheet},
    Player, Vector2,
};

#[derive(Deserialize)]
#[repr(C)]
struct WorldStructure {
    layout_tile_width: u32,
    layout_stride: u32,
    layout: Vec<u32>,

    ceiling_idx: u32,
    floor_idx: u32,

    //Sprite template name, position, angle
    sprites: Vec<(String, Vector2<f32>, f32)>,
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

        data
    }
}

const RES_MAPS: &str = "./res/maps/";

pub struct World {
    identifier: String,
    structure: WorldStructure,
    sprite_registry: SpriteRegistry,
    _ssbo: SSBO,

    resolution: (u32, u32),
    framebuffer: Framebuffer,
    gpu_slice_ssbo: SSBO,
    gpu_caf_ssbo: SSBO,
    gpu_z_ssbo: SSBO,

    pre_cf_shader: Shader,
    cf_shader: Shader,
    pre_walls_shader: Shader,
    walls_shader: Shader,
    pre_sprite_shader: Shader,
    sprite_shader: Shader,
}

impl World {
    pub fn load(
        identifier: &str,
        resolution: (u32, u32),
        player: &Player,
    ) -> Result<World, Box<dyn std::error::Error>> {
        let full_path = RES_MAPS.to_owned() + identifier + "/";

        let layout_path = String::from(full_path.clone() + "layout.ron");
        let layout_file_content = fs::read_to_string(layout_path)?;
        let layout: WorldStructure = ron::from_str(&layout_file_content)?;

        let world_tile_sheet_path = OsString::from(full_path.clone() + "sheet.ron");
        let world_tile_sheet_asset = SpriteAsset::load(&world_tile_sheet_path)?;
        let world_tile_sheet = Tilesheet::from(&world_tile_sheet_asset)?;

        let layout_gpu = layout.as_vec_for_gpu();
        let _ssbo = SSBO::from(3, &layout_gpu, gl::STATIC_DRAW);

        let sprite_registry = SpriteRegistry::create(
            &world_tile_sheet,
            &layout.sprites,
            &(full_path + "sprites"),
            player,
        )?;

        let framebuffer = Framebuffer::create(0, resolution.0 as i32, resolution.1 as i32);
        let gpu_slice_ssbo = SSBO::empty(5, 3 * resolution.0 as isize * gpu::INT, gl::DYNAMIC_DRAW);
        let gpu_caf_ssbo = SSBO::empty(6, 4 * resolution.1 as isize * gpu::FLOAT, gl::DYNAMIC_DRAW);
        let gpu_z_ssbo = SSBO::empty(7, resolution.0 as isize * gpu::DOUBLE, gl::DYNAMIC_DRAW);

        let pre_cf_shader = Shader::from(
            "./src/shader/ceiling_floor/preprocess.glsl",
            gl::COMPUTE_SHADER,
        )?;

        let cf_shader = Shader::from(
            "./src/shader/ceiling_floor/compute.glsl",
            gl::COMPUTE_SHADER,
        )?;

        let pre_walls_shader =
            Shader::from("./src/shader/walls/preprocess.glsl", gl::COMPUTE_SHADER)?;

        let walls_shader = Shader::from("./src/shader/walls/compute.glsl", gl::COMPUTE_SHADER)?;

        let pre_sprite_shader =
            Shader::from("./src/shader/sprites/preprocess.glsl", gl::COMPUTE_SHADER)?;

        let sprite_shader = Shader::from("./src/shader/sprites/compute.glsl", gl::COMPUTE_SHADER)?;

        let world = World {
            identifier: String::from(identifier),
            structure: layout,
            sprite_registry,
            _ssbo,
            resolution,
            framebuffer,
            gpu_slice_ssbo,
            gpu_caf_ssbo,
            gpu_z_ssbo,
            pre_cf_shader,
            cf_shader,
            pre_walls_shader,
            walls_shader,
            pre_sprite_shader,
            sprite_shader,
        };

        Ok(world)
    }

    pub fn identifier(&self) -> &String {
        return &self.identifier;
    }

    pub fn at(&self, x: u32, y: u32) -> &u32 {
        return &self.structure.layout[(x + y * self.structure.layout_stride) as usize];
    }

    pub fn render(&mut self, player: &Player) {
        self.sprite_registry.update(player);

        self.pre_cf_shader
            .dispatch(1, self.resolution.1, 1, gl::SHADER_STORAGE_BARRIER_BIT);

        self.cf_shader.dispatch(
            self.resolution.0,
            self.resolution.1,
            1,
            gl::SHADER_IMAGE_ACCESS_BARRIER_BIT,
        );

        self.pre_walls_shader
            .dispatch(self.resolution.0, 1, 1, gl::SHADER_STORAGE_BARRIER_BIT);

        self.walls_shader.dispatch(
            self.resolution.0,
            self.resolution.1,
            1,
            gl::SHADER_IMAGE_ACCESS_BARRIER_BIT,
        );

        let sprite_count = self.sprite_registry.count() as u32;
        self.pre_sprite_shader
            .dispatch(1, 1, sprite_count, gl::SHADER_STORAGE_BARRIER_BIT);

        for i in 0..sprite_count {
            self.sprite_shader.set_uint("sprite_idx", i);

            let preprocess = self.sprite_registry.preprocess_result(i);
            if preprocess.draw_end_x >= preprocess.draw_start_x && preprocess.transform_y > 0.0 {
                let num_groups_x = (preprocess.draw_end_x - preprocess.draw_start_x) as u32;
                let num_groups_y = (preprocess.draw_end_y - preprocess.draw_start_y) as u32;

                self.sprite_shader.dispatch(
                    num_groups_x.clamp(0, self.resolution.0),
                    num_groups_y.clamp(0, self.resolution.1),
                    1,
                    gl::SHADER_IMAGE_ACCESS_BARRIER_BIT,
                );
            }
        }

        self.framebuffer.blit();
    }
}
