use vulkano::memory::allocator::StandardMemoryAllocator;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use crate::renderer::draw_call::DrawCall;
use crate::renderer::Renderer;
use crate::renderer::model::{Model, Vertex};
use crate::renderer::shader_loader::ShaderType;
use crate::transform::Transform;

mod renderer;
mod transform;

fn main() {
    let event_loop:EventLoop<()> = EventLoop::new();
    let mut renderer:Renderer = renderer::initialize_renderer(&event_loop);


    //              --DATA HERE FOR NOW--

    let memory_allocator: StandardMemoryAllocator =
        StandardMemoryAllocator::new_default(renderer.device.clone());

    let vertices = vec![
        Vertex {
            position: [-0.5, -0.25, 0.5],
        },
        Vertex {
            position: [0.0, 0.5, 0.5],
        },
        Vertex {
            position: [0.25, -0.1, 0.5],
        },
    ];

    let transform:Transform = Transform::identity();
    let model = Model::load(&memory_allocator, vertices);

    //              --END OF DEBUG DATA--


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
                let draw_calls = vec![DrawCall{
                    transform:transform.clone(),
                    model:model.clone(),
                    vertex_shader:renderer.shader_container
                        .get_shader(ShaderType::Vertex, "direct").expect("Didn't find shader"),
                    fragment_shader:renderer.shader_container
                        .get_shader(ShaderType::Fragment, "direct").expect("Didn't find shader")}];

                renderer.submit_draw(draw_calls);
            }
            _ => (),
        }
    });
}