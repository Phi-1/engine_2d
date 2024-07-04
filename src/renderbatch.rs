use crate::texture::{Texture, TextureAtlas};

struct RenderTarget<'a>
{
    position: (f32, f32),
    size: (f32, f32),
    angle_degrees: f32,
    texture: &'a Texture
}
// FIXME: still stores texture twice
struct TextureAtlasRenderTarget<'a>
{
    render_target: RenderTarget<'a>,
    texture_atlas: &'a TextureAtlas 
}

pub struct RenderBatch<'a>
{
    targets: Vec<RenderTarget<'a>>,
    texture_atlas_targets: Vec<TextureAtlasRenderTarget<'a>>
}

impl<'a> RenderBatch<'a>
{
    pub fn render(&mut self, position: (f32, f32), size: (f32, f32), angle_degrees: f32, texture: &'a Texture)
    {
        self.targets.push(RenderTarget { position, size, angle_degrees, texture });
    }

    pub fn render_from_atlas(&mut self, position: (f32, f32), size: (f32, f32), angle_degrees: f32, texture_atlas: &TextureAtlas, texture_index: u16)
    {
        self.targets.push(TextureAtlasRenderTarget 
            {
                render_target: RenderTarget { position, size, angle_degrees, texture},
                texture_atlas
            });
    }
}