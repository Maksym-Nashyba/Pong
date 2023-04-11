mod game_object;
mod transform;
mod behaviours;

use std::sync::Arc;
use vulkanus_3d::renderer::draw_call::DrawCall;
use winit::event_loop::EventLoop;
use vulkanus_3d::renderer::Renderer;
use winit::event::{Event, WindowEvent};
use winit::window::{Window, WindowBuilder};
use crate::behaviours::RandomMovement;
use crate::game_object::{Behaviour, GameObject};

fn main(){
    let event_loop:EventLoop<()> = EventLoop::new();
    let window:Arc<Window> = Arc::new(WindowBuilder::new().build(&event_loop).unwrap());
    let mut renderer:Renderer = vulkanus_3d::innit_renderer(window.clone());
    let mut world: Vec<GameObject> = vec![];

    start(&mut world);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
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
            Event::MainEventsCleared => {
                update_game(&mut world);
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                let draw_calls: Vec<DrawCall> = prepare_draw_calls(&world);
                renderer.submit_frame(draw_calls);
            },
            _ => ()
        }
    });
}

fn start(world: &mut Vec<GameObject>){
    let mut game_object:GameObject = GameObject::new();
    let movement_behaviour:Box<dyn Behaviour> = Box::new(RandomMovement{});
    game_object.add_behaviour(movement_behaviour);
    world.push(game_object);
}

fn update_game(world: &mut Vec<GameObject>){
    for game_object in world {
        game_object.update_components();
    }
}

fn prepare_draw_calls(world: &Vec<GameObject>) -> Vec<DrawCall>{
    let mut draw_calls:Vec<DrawCall> = vec![];

    for game_object in world {
        if game_object.material.is_none() || game_object.model.is_none(){
            continue;
        }
        draw_calls.push(DrawCall{
            transform: game_object.transform.to_transformation_matrix(),
            model: game_object.model.as_ref().unwrap().clone(),
            material: game_object.material.as_ref().unwrap().clone(),
        });
    };

    return draw_calls;
}