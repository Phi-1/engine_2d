use std::{ffi::CString, fs, mem, ptr};
use gl::types::*;

pub fn create_quad_shader() -> u32
{
    let vertex_source   = fs::read_to_string("assets/shaders/quad_vs.glsl")
        .expect("Failed to load vertex shader file");
    let fragment_source = fs::read_to_string("assets/shaders/quad_fs.glsl")
        .expect("Failed to load fragment shader file");

    let vertex_shader   = compile_shader(vertex_source, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(fragment_source, gl::FRAGMENT_SHADER);

    create_shader_program(vertex_shader, fragment_shader) 
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

pub fn set_uniform_vec2(shader: u32, name: &str, vec2: &glam::f32::Vec2)
{
    unsafe
    {
        let uniform = CString::new(name.as_bytes()).unwrap();
        let location = gl::GetUniformLocation(shader, uniform.as_ptr());
        gl::Uniform2f(location, vec2.x, vec2.y);
    }
}

pub fn set_uniform_mat4(shader: u32, name: &str, mat4: &glam::f32::Mat4)
{
    unsafe
    {
        let uniform = CString::new(name.as_bytes()).unwrap();
        let location = gl::GetUniformLocation(shader, uniform.as_ptr());
        gl::UniformMatrix4fv(location, 1, gl::FALSE, mem::transmute(&mat4.to_cols_array()[0]));
    }
}