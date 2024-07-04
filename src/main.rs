extern crate glfw;
extern crate gl;
extern crate stb;
extern crate glam;

mod shader;
mod vao;
mod math;
mod texture;
mod renderbatch;

use std::time::Instant;
use glfw::Context;

// TODO: texture object for single textures
// TODO: with render batching, texture atlas object with texture id set in uniform

// TODO: set tickrate for calling provided update function, call render as often as possible
// TODO: interface for running engine from other project, option setting, provide update and render functions, start

fn main()
{
    let mut glfw = glfw::init(glfw::fail_on_errors)
        .unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));

    let (mut window, events) = glfw.create_window(800, 600, "Gaming", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    gl::load_with(|s| window.get_proc_address(s) as *const _);
    // TODO: glViewport with event listeners

    window.set_key_polling(true);
    window.make_current();
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
    //glfw.set_swap_interval(glfw::SwapInterval::None);

    unsafe { gl::ClearColor(1.0, 0.0, 1.0, 1.0) };

    // deltatime
    let mut delta_time;
    let mut last_time = Instant::now();
    let mut frame = 0;
    let mut frame_time = 0;

    // test objects
    let quad_shader = shader::create_quad_shader();
    let quad_vao    = vao::create_quad_vao();
    let texture     = texture::create_texture("gapple.png");
    let model       = math::create_model_matrix((50.0, 50.0), (32.0, 32.0), 45.0);
    let mut offsets: [(f32, f32); 128] = [(0.0, 0.0); 128];
    for i in 0..128 
    {
        offsets[i] = (i as f32, 0.0);
    }
    let projection  = math::create_projection_matrix();

    while !window.should_close()
    {
        // Deltatime
        delta_time = last_time.elapsed().as_nanos();
        last_time = Instant::now();
        frame_time += delta_time;
        frame += 1;
        if frame_time >= 1e9 as u128 
        {
            window.set_title(format!("FPS: {}", frame).as_str());
            frame_time -= 1e9 as u128;
            frame = 0;
        }

        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };

        // Rendering
        unsafe
        {
            gl::UseProgram(quad_shader);
            gl::BindVertexArray(quad_vao);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::Uniform1i(gl::GetUniformLocation(quad_shader, "texData".as_ptr() as *const i8), 0);
            shader::set_uniform_mat4(quad_shader, "model", &model);
            shader::set_uniform_mat4(quad_shader, "projection", &projection);
            for i in 0..offsets.len()
            {
                shader::set_uniform_vec2(quad_shader, format!("offsets[{}]", i).as_str(), &glam::f32::Vec2::new(offsets[i].0, offsets[i].1));
            }

            gl::DrawElementsInstanced(gl::TRIANGLES, 6, gl::UNSIGNED_BYTE, std::ptr::null(), 128);
        }

        // Events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events)
        {
            handle_window_event(&mut window, event);
        }

        window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent)
{
    match event
    {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) =>
        {
            window.set_should_close(true)
        }
        _ => {}
    }
}