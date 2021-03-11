#[macro_use]
extern crate glium;

use winit::dpi::LogicalSize;
use glium::{
    Display, Surface,
    glutin::{
        ContextBuilder,
        window::WindowBuilder,
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop}
    },
};

mod shaders;
mod text_renderer;

fn main() {
    let event_loop = EventLoop::new();
    let win_builder = WindowBuilder::new()
        .with_title("Rusttype Text Rendering w/ OpenGL")
        .with_inner_size(LogicalSize::new(800, 600))
        .with_resizable(false);
    let ctx_builder = ContextBuilder::new();
    let display = Display::new(win_builder, ctx_builder, &event_loop).unwrap();

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
        frame.clear_color(0.8, 0.7, 0.8, 1.0);

        // frame.draw(vertex_buffer: V, index_buffer: I, program: &Program, uniforms: &U, draw_parameters: &DrawParameters<'_>)
        frame.finish().unwrap();
    });
}
