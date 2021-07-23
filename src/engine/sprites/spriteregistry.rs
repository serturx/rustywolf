use crate::engine::sprites::spriteasset::SpriteAsset;
use crate::engine::sprites::Sprite;
use crate::engine::{Player, Vector2};
use crate::gpu::{TextureSampler, SSBO};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{c_void, OsString};
use std::fs;
use std::path::Path;
use std::rc::Rc;

use super::sprite::{SpritePreprocessResult, SpriteTemplate};
use super::tilesheet::{Tilesheet, VecTilesheetExt};

//SpriteAsset -> Tilesheet  -> entry in registry

pub struct SpriteRegistry {
    _sampler: TextureSampler,
    sprites_ssbo: SSBO,
    preprocess_ssbo: SSBO,
    gpu_capacity: isize,

    sprite_templates: HashMap<OsString, SpriteTemplate>,
    sprite_instances: Vec<Rc<RefCell<Sprite>>>,
}

impl SpriteRegistry {
    pub fn create(
        world_tile_sheet: &Tilesheet,
        world_sprites: &Vec<(String, Vector2<f32>, f32)>,
        sprite_folder_path: &String,
        player: &Player,
    ) -> Result<SpriteRegistry, Box<dyn std::error::Error>> {
        //Sprites folder
        let sprite_files = Self::ron_files(sprite_folder_path)?;
        let sprite_assets = Self::ron_to_sprite_asset(&sprite_files);
        let tile_sheets = Self::assets_to_tilesheets(sprite_assets);

        let width = tile_sheets.max_width().max(world_tile_sheet.tile_width());
        let height = tile_sheets.max_height().max(world_tile_sheet.tile_height());

        //Initialize sampler with worlds own sprites
        let sampler = TextureSampler::from(
            4,
            width,
            height,
            world_tile_sheet.tile_count() + tile_sheets.total_tile_count(),
            std::ptr::null() as *const c_void,
        );

        sampler.update(
            0,
            0,
            0,
            world_tile_sheet.tile_width(),
            world_tile_sheet.tile_height(),
            world_tile_sheet.tile_count(),
            world_tile_sheet.tile(0),
        );

        //Add sprite templates
        let mut sprite_templates: HashMap<OsString, SpriteTemplate> = HashMap::new();

        let mut location = world_tile_sheet.tile_count();
        for i in 0..tile_sheets.len() {
            let sprite_name = Path::new(&sprite_files[i])
                .file_stem()
                .unwrap()
                .to_os_string();

            sprite_templates.insert(
                sprite_name,
                SpriteTemplate::new(
                    location,
                    tile_sheets[i].columns() - 1,
                    tile_sheets[i].rows(),
                    tile_sheets[i].tile_width(),
                    tile_sheets[i].tile_height(),
                ),
            );

            sampler.update(
                0,
                0,
                location,
                tile_sheets[i].tile_width(),
                tile_sheets[i].tile_height(),
                tile_sheets[i].tile_count(),
                tile_sheets[i].tile(0),
            );

            location += tile_sheets[i].tile_count();
        }

        let sprites_ssbo = SSBO::empty(8, 0, gl::DYNAMIC_DRAW);
        let preprocess_ssbo = SSBO::empty(9, 0, gl::DYNAMIC_DRAW);
        let gpu_capacity: isize = 1;

        let mut registry = SpriteRegistry {
            _sampler: sampler,
            sprites_ssbo,
            preprocess_ssbo,
            gpu_capacity,
            sprite_templates,
            sprite_instances: Vec::new(),
        };

        let mut sprite_instances: Vec<Rc<RefCell<Sprite>>> = Vec::new();
        for tup in world_sprites {
            match registry.instantiate(
                &OsString::from(&tup.0),
                &tup.1,
                &Self::angle_to_vector(tup.2),
            ) {
                Some(sprite) => sprite_instances.push(sprite),
                None => {}
            }
        }

        registry.sprite_instances = sprite_instances;

        registry.update(player);

        Ok(registry)
    }

    pub fn instantiate(
        &mut self,
        sprite_id: &OsString,
        pos: &Vector2<f32>,
        dir: &Vector2<f32>,
    ) -> Option<Rc<RefCell<Sprite>>> {
        let template = self.sprite_templates.get(sprite_id)?;
        let sprite = Rc::new(RefCell::new(Sprite::new(
            pos.clone(),
            dir.clone(),
            template,
        )));
        self.sprite_instances.push(sprite.clone());

        if self.sprite_instances.len() >= self.gpu_capacity as usize {
            self.gpu_capacity *= 2;
            self.sprites_ssbo = SSBO::empty(
                8,
                self.gpu_capacity * std::mem::size_of::<Sprite>() as isize,
                gl::DYNAMIC_DRAW,
            );
            self.preprocess_ssbo = SSBO::empty(
                9,
                self.gpu_capacity * std::mem::size_of::<SpritePreprocessResult>() as isize,
                gl::DYNAMIC_DRAW,
            );
        }

        Some(sprite)
    }

    pub fn preprocess_result(&self, index: u32) -> SpritePreprocessResult {
        self.preprocess_ssbo.retrieve::<SpritePreprocessResult>(
            index as isize * std::mem::size_of::<SpritePreprocessResult>() as isize,
        )
    }

    pub fn update(&mut self, player: &Player) {
        self.sort(player);
        for i in 0..self.sprite_instances.len() {
            let s = self.sprite_instances[i].borrow_mut();

            self.sprites_ssbo
                .update(&*s, (i * std::mem::size_of::<Sprite>()) as isize);
        }
    }

    fn sort(&mut self, player: &Player) {
        self.sprite_instances.sort_by(|a, b| {
            let a = a.borrow_mut();
            let b = b.borrow_mut();

            let da = a.position().dist_sq(player.position());
            let db = b.position().dist_sq(player.position());

            db.partial_cmp(&da).unwrap()
        });
    }

    pub fn count(&self) -> usize {
        self.sprite_instances.len()
    }

    fn angle_to_vector(angle: f32) -> Vector2<f32> {
        Vector2::new(angle.to_radians().cos(), angle.to_radians().sin())
    }

    fn assets_to_tilesheets(assets: Vec<SpriteAsset>) -> Vec<Tilesheet> {
        let mut tile_sheets: Vec<Tilesheet> = Vec::new();
        for asset in assets {
            match Tilesheet::from(&asset) {
                Ok(sheet) => tile_sheets.push(sheet),
                Err(e) => eprintln!(
                    "Cant create tilesheet '{}' because '{}'",
                    asset.file,
                    e.to_string()
                ),
            }
        }

        tile_sheets
    }

    fn ron_to_sprite_asset(files: &Vec<OsString>) -> Vec<SpriteAsset> {
        let mut assets: Vec<SpriteAsset> = Vec::new();
        for sprite_file in files {
            match SpriteAsset::load(sprite_file) {
                Ok(asset) => assets.push(asset),
                Err(e) => eprintln!(
                    "Cant create asset '{:?}' because '{}'",
                    sprite_file,
                    e.to_string()
                ),
            }
        }

        assets
    }

    fn ron_files(path: &String) -> Result<Vec<OsString>, Box<dyn std::error::Error>> {
        let dir = fs::read_dir(path)?;
        let mut files: Vec<OsString> = Vec::new();
        for dir_entry in dir {
            match dir_entry {
                Ok(entry) => match entry.path().extension() {
                    Some(ext) => {
                        if ext == "ron" {
                            let mut s = OsString::from(path);
                            s.push("/");
                            s.push(entry.file_name());
                            files.push(s);
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(files)
    }
}
