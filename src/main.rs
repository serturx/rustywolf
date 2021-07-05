mod gpu;
mod localisation;
mod player;
mod settings;
mod vector;
mod world;

use glfw::{Action, Context, Key};
use std::time::Instant;
use world::World;

use localisation::I18n;
use player::Player;
use settings::Settings;
use vector::Vector2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

    let settings = Settings::load();

    let (mut window, events) = glfw
        .create_window(
            settings.resolution.0,
            settings.resolution.1,
            "Raster",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");

    window.set_key_polling(true);
    window.make_current();
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    if window.uses_raw_mouse_motion() {
        window.set_raw_mouse_motion(true);
    }

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    let i18n = I18n::from("en_GB")?;

    let mut player = Player::from(Vector2::new(1.0, 1.0));

    let map = world::World::load("test_map_1")?;
    println!("Playing {}", i18n.get_translation(map.identifier()));

    let pre_cf_shader = gpu::Shader::from(
        "./src/shader/ceiling_floor/preprocess.glsl",
        gl::COMPUTE_SHADER,
    )?;

    let cf_shader = gpu::Shader::from(
        "./src/shader/ceiling_floor/compute.glsl",
        gl::COMPUTE_SHADER,
    )?;

    let pre_walls_shader =
        gpu::Shader::from("./src/shader/walls/preprocess.glsl", gl::COMPUTE_SHADER)?;

    let walls_shader = gpu::Shader::from("./src/shader/walls/compute.glsl", gl::COMPUTE_SHADER)?;

    gpu::debug::init();
    let gpu_framebuffer = gpu::Framebuffer::create(
        0,
        settings.resolution.0 as i32,
        settings.resolution.1 as i32,
    );

    let _gpu_settings = gpu::SSBO::from(1, &settings, gl::STATIC_DRAW);
    let gpu_player = gpu::SSBO::from(2, &player, gl::DYNAMIC_DRAW);

    let map_data = map.as_vec_for_gpu();
    let _gpu_map = gpu::SSBO::from(3, &map_data, gl::STATIC_DRAW);

    let (sheet, tile_width) = map.sampler_data();
    let _sampler = gpu::TextureSampler::from(4, sheet, tile_width);

    let _gpu_slice_data = gpu::SSBO::empty(
        5,
        3 * settings.resolution.0 as isize * gpu::INT32,
        gl::DYNAMIC_DRAW,
    );
    let _gpu_caf_data = gpu::SSBO::empty(
        6,
        4 * settings.resolution.1 as isize * gpu::FLOAT32,
        gl::DYNAMIC_DRAW,
    );

    let mut delta_time: f32;
    let mut now = Instant::now();

    let mut mouse_delta = Vector2::new(0.0 as f32, 0.0);
    let mut mouse_pos = Vector2::new(0.0 as f32, 0.0);

    while !window.should_close() {
        delta_time = now.elapsed().as_secs_f32();
        now = Instant::now();

        let (mx, my) = window.get_cursor_pos();
        mouse_delta.set(mx as f32 - mouse_pos.x, my as f32 - mouse_pos.y);
        mouse_pos.set(mx as f32, my as f32);

        player.rotate_by_mouse(&mouse_delta, delta_time);
        gpu_player.update(&player);

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event, &mut player, &map, delta_time);
        }

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        pre_cf_shader.dispatch(
            1,
            settings.resolution.1 as u32,
            1,
            gl::SHADER_STORAGE_BARRIER_BIT,
        );

        cf_shader.dispatch(
            settings.resolution.0 as u32,
            settings.resolution.1 as u32,
            1,
            gl::SHADER_IMAGE_ACCESS_BARRIER_BIT,
        );

        pre_walls_shader.dispatch(
            settings.resolution.0 as u32,
            1,
            1,
            gl::SHADER_STORAGE_BARRIER_BIT,
        );

        walls_shader.dispatch(
            settings.resolution.0 as u32,
            settings.resolution.1 as u32,
            1,
            gl::SHADER_IMAGE_ACCESS_BARRIER_BIT,
        );

        gpu_framebuffer.blit();
        window.swap_buffers();
    }

    Ok(())
}

fn handle_window_event(
    window: &mut glfw::Window,
    event: glfw::WindowEvent,
    player: &mut Player,
    world: &World,
    delta_time: f32,
) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_cursor_mode(glfw::CursorMode::Normal);
        }
        glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
            player.step_forward(world, delta_time);
        }
        glfw::WindowEvent::Key(Key::S, _, Action::Press, _) => {
            player.step_backward(world, delta_time);
        }
        glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => {
            player.step_right(world, delta_time);
        }
        glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
            player.step_left(world, delta_time);
        }
        _ => {}
    }
}
