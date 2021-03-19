
use glium::backend::Facade;
use glium::{
    texture::{ClientFormat, MipmapsOption, RawImage2d, Texture2d, UncompressedFloatFormat},
    Program, Surface, Frame, VertexBuffer, IndexBuffer, 
    index::PrimitiveType
};
use rusttype::{gpu_cache::*, *};
use std::borrow::Cow;

pub struct RenderedText {
    texture: Texture2d,
    size: (u32, u32),
}

pub struct FontRenderer<'a> {
    font: Font<'a>,
    cache: Cache<'a>,
    cache_tex: Texture2d,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
mod implement_vertex {
    use super::Vertex;
    implement_vertex!(Vertex, position, tex_coords);
}

impl FontRenderer<'_> {
    pub fn new<T>(display: &T) -> Self
    where
        T: Facade,
    {
        let font = include_bytes!("../res/LinLibertine_R.ttf");
        let font = Font::try_from_bytes(font).unwrap();
        let cache = gpu_cache::Cache::builder().dimensions(1024, 1024).build();

        let cache_tex = Texture2d::with_format(
            display,
            RawImage2d {
                data: Cow::Owned(vec![128u8; 1024 * 1024]),
                width: 1024,
                height: 1024,
                format: ClientFormat::U8,
            },
            UncompressedFloatFormat::U8,
            MipmapsOption::NoMipmap,
        )
        .unwrap();

        FontRenderer {
            font,
            cache,
            cache_tex,
        }
    }

    pub fn get_glyph<T>(&self, id: T) -> Glyph<'_>
    where
        T: IntoGlyphId,
    {
        self.font.glyph(id)
    }

    /// Renders text to the specified point on the screen. Only use positions from -1 to 1
    pub fn render_text<'a, T>(
        &mut self,
        text: &'a str,
        pos: (f32, f32),
        frame: &mut Frame,
        display: &T,
        program: &Program,
    ) -> Result<(), ()>
    where
        T: Facade,
    {
        // TODO add scale to arguments or check from Winit display
        let scale = Scale { x: 100.0, y: 100.0 };
        let glyphs: Vec<_> = self
            .font
            .layout(text, scale, Point { x: pos.0, y: pos.1 })
            .collect();


        // SAFETY: rusttype crate expects the text to exist as long as the cache whenever it is passed into
        // cache.queue_glyph(). This is irrelevant here because the glyph is getting cached as soon as it
        // is added later in the code. This code is necessary to evade certain lifetime requirements.
        unsafe {
            for g in &glyphs {
                (*(&self.cache as *const Cache<'_> as *mut Cache<'_>)).queue_glyph(0, g.clone());
            }
        }

        {
            let cache_tex = &self.cache_tex;
            let cache = &mut self.cache;
            cache
                .cache_queued(|rect, data| {
                    // Not sure why I have to get the main mipmap first
                    // since the texture doesn't have a mipmap.
                    // Maybe writing to the texture directly would cause errors?
                    // It's probably just an API thing.
                    cache_tex.main_level().write(
                        glium::Rect {
                            left: rect.min.x,
                            bottom: rect.min.y,
                            width: rect.width(),
                            height: rect.height(),
                        },
                        RawImage2d {
                            data: Cow::Borrowed(data),
                            width: rect.width(),
                            height: rect.height(),
                            format: ClientFormat::U8,
                        },
                    );
                })
                .unwrap();
                cache.clear_queue();
        }

        let (scr_width, scr_height) = {
            let (w, h) = frame.get_dimensions();
            (w as f32, h as f32)
        };

        let vertices: Vec<_> = glyphs
            .iter()
            .filter_map(|g| self.cache.rect_for(0, g).ok().flatten())
            .flat_map(|(uv_rect, screen_rect)| {
                let gl_rect = Rect {
                    min: Point {
                        x: screen_rect.min.x as f32 / scr_width,
                        y: screen_rect.min.y as f32 / scr_height,
                    },
                    max: Point {
                        x: screen_rect.max.x as f32 / scr_width,
                        y: screen_rect.max.y as f32 / scr_height,
                    },
                };

                // This can be brought down to 4 vertices
                // if I add an index buffer
                vec![
                    Vertex {
                        position: [gl_rect.min.x, gl_rect.max.y],
                        tex_coords: [uv_rect.min.x, uv_rect.max.y],
                    },
                    Vertex {
                        position: [gl_rect.min.x, gl_rect.min.y],
                        tex_coords: [uv_rect.min.x, uv_rect.min.y],
                    },
                    Vertex {
                        position: [gl_rect.max.x, gl_rect.min.y],
                        tex_coords: [uv_rect.max.x, uv_rect.min.y],
                    },
                    Vertex {
                        position: [gl_rect.max.x, gl_rect.min.y],
                        tex_coords: [uv_rect.max.x, uv_rect.min.y],
                    },
                    Vertex {
                        position: [gl_rect.max.x, gl_rect.max.y],
                        tex_coords: [uv_rect.max.x, uv_rect.max.y],
                    },
                    Vertex {
                        position: [gl_rect.min.x, gl_rect.max.y],
                        tex_coords: [uv_rect.min.x, uv_rect.max.y],
                    },
                ]
            })
            .collect();

        let vert_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
        let uniform = uniform! {
            tex: self.cache_tex.sampled()
        };


        frame
            .draw(
                &vert_buffer,
                glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                program,
                &uniform,
                &glium::DrawParameters {
                    blend: glium::Blend::alpha_blending(),
                    ..Default::default()
                },
            )
            .unwrap();

        Ok(())
    }

    pub fn debug_texture<T: Facade>(&mut self, frame: &mut Frame, facade: &T, program: &Program) {

        let data = [
            // Top left
            Vertex { position: [-1.0, 1.0], tex_coords: [0.0, 1.0] },
            // Bottom left
            Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
            // Top right
            Vertex { position: [1.0, 1.0], tex_coords: [1.0, 1.0] },
            // Bottom right
            Vertex { position: [1.0, -1.0], tex_coords: [1.0, 0.0] },
        ];
        let vbo = VertexBuffer::new(facade, &data).unwrap();
        let ibo = IndexBuffer::new(facade, PrimitiveType::TrianglesList, &[0 as u16, 2, 1, 3]).unwrap();

        let uniforms = uniform! {
            tex: self.cache_tex.sampled()
        };

        frame.draw(&vbo, &ibo, program, &uniforms, &glium::DrawParameters::default()).unwrap();
    }
}



pub fn text_rendering_program<T: Facade>(facade: &T) -> glium::Program {
    glium::Program::from_source(facade, VERT_SHADER, FRAG_SHADER, None).unwrap()
}

const VERT_SHADER: &str = r"
#version 330 core

in vec2 position;
in vec2 tex_coords;

out vec2 TexCoords;

void main()
{
    gl_Position = vec4(position.xy, 0.0, 1.0);
    TexCoords = tex_coords;
}
";

const FRAG_SHADER: &str = r"
#version 330 core

in vec2 TexCoords;
out vec4 color;

uniform sampler2D tex;

void main()
{
    color = texture(tex, TexCoords);
}
";