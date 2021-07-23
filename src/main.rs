mod engine;
mod gpu;

use crate::engine::{player, I18n, Player, Settings, Vector2, World};

use glfw::{Action, Context, Key};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

    let mut settings = Settings::load();

    let (mut window, events) = glfw
        .create_window(
            settings.resolution().0,
            settings.resolution().1,
            "Raster",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");

    window.set_key_polling(true);
    window.make_current();
    //window.set_cursor_mode(glfw::CursorMode::Disabled);
    if window.uses_raw_mouse_motion() {
        window.set_raw_mouse_motion(true);
    }

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    settings.copy_to_gpu();

    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    let i18n = I18n::from(settings.language())?;

    let mut player = Player::from(Vector2::new(2.0, 2.0));

    let mut world = World::load("test_map_2", *settings.resolution(), &player)?;
    println!("Playing {}", i18n.get_translation(world.identifier()));

    let mut delta_time: f32;
    let mut now = Instant::now();

    let mut mouse_delta = Vector2::new(0.0 as f32, 0.0);
    let mut mouse_pos = Vector2::new(0.0 as f32, 0.0);

    while !window.should_close() {
        delta_time = now.elapsed().as_secs_f32();
        now = Instant::now();

        //println!("{}", 1.0 / delta_time);

        let (mx, my) = window.get_cursor_pos();
        mouse_delta.set(mx as f32 - mouse_pos.x, my as f32 - mouse_pos.y);
        mouse_pos.set(mx as f32, my as f32);

        player.update_position(&world, delta_time);
        player.rotate_by_mouse(&mouse_delta, delta_time);
        player.copy_to_gpu();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event, &mut player);
        }

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        world.render(&player);
        window.swap_buffers();
    }

    Ok(())
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent, player: &mut Player) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_cursor_mode(glfw::CursorMode::Normal);
        }

        glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
            player.start_movement(player::FORWARDS);
        }
        glfw::WindowEvent::Key(Key::W, _, Action::Release, _) => {
            player.end_movement(player::FORWARDS);
        }

        glfw::WindowEvent::Key(Key::S, _, Action::Press, _) => {
            player.start_movement(player::BACKWARDS);
        }
        glfw::WindowEvent::Key(Key::S, _, Action::Release, _) => {
            player.end_movement(player::BACKWARDS);
        }

        glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => {
            player.start_movement(player::RIGHT);
        }
        glfw::WindowEvent::Key(Key::D, _, Action::Release, _) => {
            player.end_movement(player::RIGHT);
        }

        glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
            player.start_movement(player::LEFT);
        }
        glfw::WindowEvent::Key(Key::A, _, Action::Release, _) => {
            player.end_movement(player::LEFT);
        }
        _ => {}
    }
}
