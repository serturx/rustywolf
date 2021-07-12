use crate::{
    gpu::{self, SSBO},
    player::Player,
    vector::Vector2,
    world::World,
};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[repr(C)]
pub struct Sprite {
    position: Vector2<f32>,
    texture_idx: u32,
}

impl Sprite {
    pub fn new(position: Vector2<f32>, texture_idx: u32) -> Sprite {
        Sprite {
            position,
            texture_idx,
        }
    }
}

///This struct is only for ssbo size calculation
#[allow(dead_code)]
#[repr(C)]
struct SpritePreprocessResult {
    sprite_width: i32,
    sprite_height: i32,

    draw_start_y: i32,
    draw_end_y: i32,
    draw_start_x: i32,
    draw_end_x: i32,

    sprite_screen_x: i32,
    transform_y: f64,
}

pub struct Sprites {
    _gpu_capacity: isize,
    sprites: Vec<Sprite>,
    _sprites_ssbo: SSBO,
    _preprocess_ssbo: SSBO,
}

impl Sprites {
    pub fn new(world: &World, player: &Player) -> Sprites {
        let sprites = world.sprites().clone();

        let _sprites_ssbo = gpu::SSBO::empty(
            8,
            (sprites.len() * std::mem::size_of::<Sprite>()) as isize,
            gl::DYNAMIC_DRAW,
        );

        let _preprocess_ssbo = gpu::SSBO::empty(
            9,
            (sprites.len() * std::mem::size_of::<SpritePreprocessResult>()) as isize,
            gl::DYNAMIC_DRAW,
        );

        let mut sprites = Sprites {
            _gpu_capacity: sprites.len() as isize,
            sprites,
            _sprites_ssbo,
            _preprocess_ssbo,
        };

        sprites.sort(player);
        sprites.force_gpu_update();

        sprites
    }

    pub fn add(&mut self, sprite: Sprite) {
        self.sprites.push(sprite);

        if self.sprites.len() >= self._gpu_capacity as usize {
            self._gpu_capacity *= 2;
            self._sprites_ssbo = gpu::SSBO::empty(
                8,
                self._gpu_capacity * std::mem::size_of::<Sprite>() as isize,
                gl::DYNAMIC_DRAW,
            );
            self._preprocess_ssbo = gpu::SSBO::empty(
                9,
                self._gpu_capacity * std::mem::size_of::<SpritePreprocessResult>() as isize,
                gl::DYNAMIC_DRAW,
            );
        }

        self.force_gpu_update();
    }

    ///Manually triggers copying the sprite vector to the gpu
    //Should be called whenever a sprites position or texture is changed or a sprite gets removed/added
    pub fn force_gpu_update(&self) {
        self._sprites_ssbo.update(&self.sprites);
    }

    pub fn update(&mut self, player: &Player) {
        self.sort(player);
        self.force_gpu_update();
    }

    fn sort(&mut self, player: &Player) {
        self.sprites.sort_by(|a, b| {
            let da = a.position.dist_sq(player.position());
            let db = b.position.dist_sq(player.position());

            db.partial_cmp(&da).unwrap()
        });
    }

    pub fn len(&self) -> usize {
        return self.sprites.len();
    }
}
