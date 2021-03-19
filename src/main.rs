#[macro_use]
extern crate glium;

use glium::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder,
    },
    index::PrimitiveType,
    texture::{MipmapsOption, RawImage2d, Texture2d, UncompressedFloatFormat},
    Display, DrawParameters, IndexBuffer, Surface, VertexBuffer,
};
use std::borrow::Cow;
use winit::dpi::LogicalSize;

mod text_renderer;

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

mod implement_vertex {
    use super::Vertex;
    implement_vertex!(Vertex, position, tex_coords);
}

fn main() {
    let event_loop = EventLoop::new();
    let win_builder = WindowBuilder::new()
        .with_title("Rusttype Text Rendering w/ OpenGL")
        .with_inner_size(LogicalSize::new(800, 600))
        .with_resizable(false);
    let ctx_builder = ContextBuilder::new();
    let display = Display::new(win_builder, ctx_builder, &event_loop).unwrap();

    let mut text_renderer = text_renderer::FontRenderer::new(&display);
    let program = text_renderer::text_rendering_program(&display);

    let raw_img = get_floppa_texture().unwrap();
    let floppa_tex = Texture2d::with_format(
        &display,
        raw_img,
        UncompressedFloatFormat::U8U8U8U8,
        MipmapsOption::NoMipmap,
    )
    .unwrap();
    let tex_verts = vec![
        Vertex {
            position: [-1.0, -1.0],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [-1.0, 1.0],
            tex_coords: [0.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [1.0, -1.0],
            tex_coords: [1.0, 0.0],
        },
    ];

    let floppa_vbo = VertexBuffer::new(&display, &tex_verts).unwrap();
    let floppa_ibo =
        IndexBuffer::new(&display, PrimitiveType::TriangleStrip, &[1 as u16, 2, 0, 3]).unwrap();

    event_loop.run(move |event, _window_target, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
        let mut frame = display.draw();
        frame.clear_color(0.4, 0.7, 0.8, 1.0);

        let floppa_uniforms = uniform! {
            tex: floppa_tex.sampled(),
        };
        // Draw debug floppa texture
        frame.draw(
            &floppa_vbo,
            &floppa_ibo,
            &program,
            &floppa_uniforms,
            &DrawParameters {
                blend: glium::Blend::alpha_blending(),
                ..Default::default()
            },
        ).unwrap();

        // text_renderer.debug_texture(&mut frame, &display, &program);

        text_renderer
            .render_text("text: &'a str", (0.5, 0.5), &mut frame, &display, &program)
            .unwrap();
        // frame.draw(vertex_buffer: V, index_buffer: I, program: &Program, uniforms: &U, draw_parameters: &DrawParameters<'_>)

        frame.finish().unwrap();
    });
}

fn get_floppa_texture<'a>() -> Result<RawImage2d<'a, u8>, Box<dyn std::error::Error>> {
    use image::io::Reader;
    use std::io::Cursor;

    let floppa_bytes = include_bytes!("../res/floppa.jpg");
    let reader = Reader::new(Cursor::new(floppa_bytes))
        .with_guessed_format()?
        .decode()?.flipv();
    let image = reader.to_rgba8();
    let (h, w) = (image.height(), image.width());
    let raw = image.into_raw();

    let tex = RawImage2d::from_raw_rgba(raw, (h, w));

    Ok(tex)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_does_decode() -> Result<(), Box<dyn std::error::Error>> {
        get_floppa_texture()?;
        Ok(())
    }
}
