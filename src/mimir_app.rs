use std::sync::Arc;
use std::time::{Duration, Instant};

use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;

use crate::graphics::State;

const TARGET_FPS: f64 = 60.0;

pub struct MimirApp {
    // window: Option<Window>,
    state: Option<State>,
    last_frame_time: Instant,
    accumulated_time: Duration,
}

impl MimirApp {
    pub fn new() -> Self {
        Self {
            // window: None,
            state: None,
            last_frame_time: Instant::now(),
            accumulated_time: Duration::ZERO,
        }
    }
}

impl ApplicationHandler for MimirApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }
        let attributes = Window::default_attributes()
            .with_title("Mimir Game Engine")
            .with_inner_size(winit::dpi::LogicalSize::new(800, 600));

        let window = Arc::new(
            event_loop
                .create_window(attributes)
                .expect("failed to create window"),
        );
        self.state = Some(State::new(window));
        self.last_frame_time = Instant::now();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(state) = &mut self.state {
            if _window_id == state.window_id() {
                match event {
                    WindowEvent::CloseRequested => event_loop.exit(),
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => event_loop.exit(),
                    WindowEvent::Resized(new_size) => state.resize(new_size),
                    WindowEvent::RedrawRequested => if state.render() {},
                    _ => (),
                }
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        let target_frame_duration = Duration::from_secs_f64(1.0 / TARGET_FPS);
        let now = Instant::now();
        let delta_time = now - self.last_frame_time;
        self.last_frame_time = now;
        self.accumulated_time += delta_time;

        if self.accumulated_time > target_frame_duration {
            if let Some(state) = &mut self.state {
                state.update();
                state.request_redraw();
                self.accumulated_time -= target_frame_duration;
            }
        }
    }
}
