mod app;
mod audio;
mod camera;
mod components;
mod engine;
mod input;
mod scenes;
mod state;

use winit::event_loop::EventLoop;

use crate::app::MimirApp;

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().expect("failed to init EventLoop");
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut app = MimirApp::new();

    event_loop.run_app(&mut app).expect("Failed to run app");
}
