use std::{ffi::c_void, mem, ptr};
use gl::types::*;

fn create_quad_vertices_and_indices() -> ([f32; 16], [u8; 6])
{
    ([  // pos      // tex
        -0.5, -0.5, 0.0, 0.0,
         0.5,  0.5, 1.0, 1.0,
        -0.5,  0.5, 0.0, 1.0,
         0.5, -0.5, 1.0, 0.0
    ],
    [
        0, 1, 2,
        0, 3, 1
    ])
}

pub fn create_quad_vao() -> u32
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

        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, (mem::size_of::<f32>() * 4) as i32, ptr::null());
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, (mem::size_of::<f32>() * 4) as i32, ((mem::size_of::<f32>() * 2) as i32) as *const c_void);
        gl::EnableVertexAttribArray(1);

        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    };

    vao
}