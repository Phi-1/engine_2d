use std::{fs, mem};

use gl::types::{GLint, GLsizei};

pub struct Texture
{
    opengl_handle: u32,
    width: u16,
    height: u16
}

pub struct TextureAtlas
{
    texture: Texture,
    interval_x: u16,
    interval_y: u16,
}

impl Texture
{
    pub fn from_file(filename: &str) -> Self 
    {
        // TODO: better error handling
        let file = fs::File::open(format!("assets/images/{}", filename))
            .expect(format!("Couldn't open texture file {}", filename).as_str());
        let mut buf_reader = std::io::BufReader::new(file);

        stb::image::stbi_set_flip_vertically_on_load(true);
        // TODO: here as well
        let (image_info, image_data) = stb::image::stbi_load_from_reader(&mut buf_reader, stb::image::Channels::RgbAlpha)
            .expect(format!("Error loading texture {}", filename).as_str());

        let mut handle = 0;

        unsafe
        {
            gl::GenTextures(1, &mut handle);
            gl::BindTexture(gl::TEXTURE_2D, handle);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT as GLint);

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as GLint, image_info.width as GLsizei, image_info.height as GLsizei, 0, gl::RGBA, gl::UNSIGNED_BYTE, mem::transmute(&image_data.as_slice()[0]));

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        
        Texture
        {
            opengl_handle: handle,
            width: image_info.width as u16,
            height: image_info.height as u16 
        } 
    }
}

impl TextureAtlas
{
    pub fn from_file(filename: &str, interval_x: u16, interval_y: u16) -> Self
    {
        let texture = Texture::from_file(filename);

        TextureAtlas
        {
            texture,
            interval_x,
            interval_y
        }
    }

    pub fn from_texture(texture: Texture, interval_x: u16, interval_y: u16) -> Self
    {
        TextureAtlas
        {
            texture,
            interval_x,
            interval_y
        }
    }
}