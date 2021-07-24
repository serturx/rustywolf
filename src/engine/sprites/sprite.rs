use crate::{engine::util::Vector2, gpu::ISSBO};

///Layout(linear on gpu)
///N: Number of different animation sprites per view-angle
///M: Number of view angles
///texture_base_index,
///     |
///Front-Idle, Front-Anim1, Front-Anim2, ... , Front-AnimN,
///Angle1-Idle, Angle1-Anim1, Angle1-Anim2, ... , Angle1-AnimN,
///Angle2-Idle, Angle2-Anim1, Angle2-Anim2, ... , Angle2-AnimN,
// ...
///AngleM-Idle, AngleM-Anim1, AngleM-Anim2, ... , AngleM-AnimN,
#[repr(C)]
pub struct Sprite {
    position: Vector2<f32>,
    direction: Vector2<f32>,

    template: SpriteTemplate,
    animation_index: i32,
    animation_tick_period: i32,
    animation_tick: i32,
}

impl ISSBO for Sprite {}

impl Sprite {
    pub fn new(
        position: Vector2<f32>,
        direction: Vector2<f32>,
        template: &SpriteTemplate,
    ) -> Sprite {
        Sprite {
            position,
            direction,
            animation_index: 0,
            template: (*template).clone(),
            animation_tick_period: 30,
            animation_tick: 0,
        }
    }

    pub fn move_by(&mut self, speed: f32, delta_time: f32) {
        self.position += self.direction * speed * delta_time;

        self.animation_tick += 1;
        if self.animation_tick != self.animation_tick_period {
            return;
        }

        if !self.animating() {
            self.start_animation();
        } else {
            self.next_frame();
        }

        self.animation_tick = 0;
    }

    pub fn position(&self) -> &Vector2<f32> {
        &self.position
    }

    fn animating(&self) -> bool {
        self.animation_index > 0
    }

    pub fn start_animation(&mut self) {
        self.animation_index = 1;
    }

    pub fn end_animation(&mut self) {
        self.animation_index = 0;
    }

    pub fn next_frame(&mut self) {
        self.animation_index += 1;

        if self.animation_index > self.template.animation_count {
            self.animation_index = 1;
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct SpriteTemplate {
    texture_base_index: i32,

    animation_count: i32,
    view_angle_count: i32,

    tile_width: i32,
    tile_height: i32,
}

impl ISSBO for SpriteTemplate {}

impl SpriteTemplate {
    pub fn new(
        texture_base_index: i32,
        animation_count: i32,
        view_angle_count: i32,
        tile_width: i32,
        tile_height: i32,
    ) -> SpriteTemplate {
        SpriteTemplate {
            texture_base_index,
            animation_count,
            view_angle_count,
            tile_width,
            tile_height,
        }
    }
}

///This struct is for retrieving intermediate sprite processing results
#[allow(dead_code)]
#[derive(Default)]
#[repr(C)]
pub struct SpritePreprocessResult {
    sprite_width: i32,
    sprite_height: i32,

    pub draw_start_y: i32,
    pub draw_end_y: i32,
    pub draw_start_x: i32,
    pub draw_end_x: i32,

    sprite_screen_x: i32,
    pub transform_y: f64,
}
