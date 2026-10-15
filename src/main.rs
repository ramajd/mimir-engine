mod ecs;
mod graphics;
mod input;
mod mimir_app;

use winit::event_loop::EventLoop;

use crate::mimir_app::MimirApp;

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().expect("failed to init EventLoop");
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut app = MimirApp::new();

    event_loop.run_app(&mut app).expect("Failed to run app");
}
