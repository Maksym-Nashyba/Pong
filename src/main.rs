use winit::event_loop::EventLoop;
use renderer::renderer::Renderer;
use winit::event::{Event, WindowEvent};

fn main(){
    let event_loop:EventLoop<()> = EventLoop::new();
    let mut renderer:Renderer = renderer::innit_renderer(&event_loop);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                control_flow.set_exit();
            },
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                renderer.on_resized();
            },
            Event::RedrawEventsCleared => {
                //renderer.submit_frame();
            },
            _ => ()
        }
    });
}