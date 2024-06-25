use std::{fs, mem};

use gl::types::{GLint, GLsizei};

pub fn create_texture(filename: &str) -> u32
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