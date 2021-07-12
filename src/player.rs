use super::gpu::ISSBO;
use super::vector::Vector2;
use super::world::World;

const ROTATION_SPEED: f32 = 0.5;
const MOVEMENT_SPEED: f32 = 5.0;

type MoveDir = usize;
pub const FORWARDS: MoveDir = 0;
pub const BACKWARDS: MoveDir = 1;
pub const RIGHT: MoveDir = 2;
pub const LEFT: MoveDir = 3;

#[repr(C)]
pub struct Player {
    position: Vector2<f32>,
    direction: Vector2<f32>,
    camera_plane: Vector2<f32>,
    movement: [bool; 4], //Forward,Backward,Right,Left
}

impl ISSBO for Player {}

impl Player {
    pub fn from(position: Vector2<f32>) -> Player {
        return Player {
            position,
            direction: Vector2::new(-0.75, 0.0),
            camera_plane: Vector2::new(0.0, 0.66),
            movement: [false, false, false, false],
        };
    }

    fn rotate(&mut self, angle: f32) {
        let old_dir_x = self.direction.x;
        self.direction.x = self.direction.x * angle.cos() - self.direction.y * angle.sin();
        self.direction.y = old_dir_x * angle.sin() + self.direction.y * angle.cos();

        let old_plane_x = self.camera_plane.x;
        self.camera_plane.x = self.camera_plane.x * angle.cos() - self.camera_plane.y * angle.sin();
        self.camera_plane.y = old_plane_x * angle.sin() + self.camera_plane.y * angle.cos();
    }

    pub fn rotate_by_mouse(&mut self, rel_mov: &Vector2<f32>, delta_time: f32) {
        self.rotate(rel_mov.x * ROTATION_SPEED * delta_time * -1.0);
    }

    pub fn update_position(&mut self, world: &World, delta_time: f32) {
        if self.movement[FORWARDS] {
            self.step_forward(world, delta_time);
        }

        if self.movement[BACKWARDS] {
            self.step_backward(world, delta_time);
        }

        if self.movement[RIGHT] {
            self.step_right(world, delta_time);
        }

        if self.movement[LEFT] {
            self.step_left(world, delta_time);
        }
    }

    pub fn start_movement(&mut self, dir: MoveDir) {
        self.movement[dir] = true;
    }

    pub fn end_movement(&mut self, dir: MoveDir) {
        self.movement[dir] = false;
    }

    fn step_forward(&mut self, world: &World, delta_time: f32) {
        let future_x = self.position.x + self.direction.x * MOVEMENT_SPEED * delta_time;
        if *world.at(future_x as u32, self.position.y as u32) == 0 {
            self.position.x = future_x;
        }

        let future_y = self.position.y + self.direction.y * MOVEMENT_SPEED * delta_time;
        if *world.at(self.position.x as u32, future_y as u32) == 0 {
            self.position.y = future_y;
        }
    }

    fn step_backward(&mut self, world: &World, delta_time: f32) {
        let future_x = self.position.x - self.direction.x * MOVEMENT_SPEED * delta_time;
        if *world.at(future_x as u32, self.position.y as u32) == 0 {
            self.position.x = future_x;
        }

        let future_y = self.position.y - self.direction.y * MOVEMENT_SPEED * delta_time;
        if *world.at(self.position.x as u32, future_y as u32) == 0 {
            self.position.y = future_y;
        }
    }

    fn step_right(&mut self, world: &World, delta_time: f32) {
        let future_x = self.position.x + self.camera_plane.x * MOVEMENT_SPEED * delta_time;
        if *world.at(future_x as u32, self.position.y as u32) == 0 {
            self.position.x = future_x;
        }

        let future_y = self.position.y + self.camera_plane.y * MOVEMENT_SPEED * delta_time;
        if *world.at(self.position.x as u32, future_y as u32) == 0 {
            self.position.y = future_y;
        }
    }

    fn step_left(&mut self, world: &World, delta_time: f32) {
        let future_x = self.position.x - self.camera_plane.x * MOVEMENT_SPEED * delta_time;
        if *world.at(future_x as u32, self.position.y as u32) == 0 {
            self.position.x = future_x;
        }

        let future_y = self.position.y - self.camera_plane.y * MOVEMENT_SPEED * delta_time;
        if *world.at(self.position.x as u32, future_y as u32) == 0 {
            self.position.y = future_y;
        }
    }

    pub fn position(&self) -> &Vector2<f32> {
        return &self.position;
    }
}
