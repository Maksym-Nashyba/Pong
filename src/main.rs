use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use crate::renderer::Renderer;

mod renderer;
mod transform;

fn main() {
    let event_loop:EventLoop<()> = EventLoop::new();
    let mut renderer:Renderer = renderer::initialize_renderer(&event_loop);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                renderer.on_resized();
            }
            Event::RedrawEventsCleared => {
                renderer.submit_draw();
            }
            _ => (),
        }
    });
}