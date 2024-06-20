extern crate glfw;
extern crate gl;
extern crate stb;

use std::{ffi::CString, fs, mem, ptr};
use gl::types::{GLchar, GLenum, GLint, GLsizei, GLsizeiptr};
use glfw::Context;

// TODO: texture object for single textures
// TODO: with render batching, texture atlas object with texture id set in uniform
fn main()
{
    let mut glfw = glfw::init(glfw::fail_on_errors)
        .unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));

    let (mut window, events) = glfw.create_window(800, 600, "Gaming", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    window.set_key_polling(true);
    window.make_current();
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    unsafe { gl::ClearColor(0.0, 0.0, 0.0, 1.0) };
    let quad_shader = create_quad_shader();
    let quad_vao    = create_quad_vao();
    let texture     = create_texture("gapple.png");

    while !window.should_close()
    {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };

        // Rendering
        unsafe
        {
            gl::UseProgram(quad_shader);
            gl::BindVertexArray(quad_vao);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::Uniform1i(gl::GetUniformLocation(quad_shader, "texData".as_ptr() as *const i8), 0);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_BYTE, std::ptr::null());
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
        _ => 
        {}
    }
}

fn create_quad_vertices_and_indices() -> ([f32; 8], [u8; 6])
{
    ([
        -1.0, -1.0, 
         1.0,  1.0,
        -1.0,  1.0,
         1.0, -1.0,
    ],
    [
        0, 1, 2,
        0, 3, 1
    ])
}

fn create_quad_vao() -> u32
{
    let mut vbo = 0;
    let mut vao = 0;
    let mut ebo = 0;

    unsafe
    {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let (vertices, indices) = create_quad_vertices_and_indices();
        gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * mem::size_of::<f32>()) as GLsizeiptr, mem::transmute(&vertices[0]), gl::STATIC_DRAW);

        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * mem::size_of::<u8>()) as GLsizeiptr, mem::transmute(&indices[0]), gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, (mem::size_of::<f32>() * 2) as i32, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    };

    vao
}

fn compile_shader(source: String, shader_type: GLenum) -> u32
{
    let shader;

    unsafe
    {
        shader = gl::CreateShader(shader_type);
        let c_str = CString::new(source.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                std::str::from_utf8(&buf)
                    .ok()
                    .expect("ShaderInfoLog not valid utf8")
            );
        }
    };

    shader
}

fn create_shader_program(vertex_shader: u32, fragment_shader: u32) -> u32
{
    let program;

    unsafe
    {
        program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                std::str::from_utf8(&buf)
                    .ok()
                    .expect("ProgramInfoLog not valid utf8")
            );
        }
    }

    program
}

fn create_quad_shader() -> u32
{
    let vertex_source   = fs::read_to_string("assets/shaders/quad_vs.glsl")
        .expect("Failed to load vertex shader file");
    let fragment_source = fs::read_to_string("assets/shaders/quad_fs.glsl")
        .expect("Failed to load fragment shader file");

    let vertex_shader   = compile_shader(vertex_source, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(fragment_source, gl::FRAGMENT_SHADER);

    create_shader_program(vertex_shader, fragment_shader) 
}

fn create_texture(filename: &str) -> u32
{
    let file = fs::File::open(format!("assets/images/{}", filename))
        .expect(format!("Couldn't open texture file {}", filename).as_str());
    let mut buf_reader = std::io::BufReader::new(file);

    stb::image::stbi_set_flip_vertically_on_load(true);
    let (image_info, image_data) = stb::image::stbi_load_from_reader(&mut buf_reader, stb::image::Channels::RgbAlpha)
        .expect(format!("Error loading texture {}", filename).as_str());

    let mut texture = 0;

    unsafe
    {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT as GLint);

        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as GLint, image_info.width as GLsizei, image_info.height as GLsizei, 0, gl::RGBA, gl::UNSIGNED_BYTE, mem::transmute(&image_data.as_slice()[0]));

        gl::BindTexture(gl::TEXTURE_2D, 0);
    }
    
    texture
}