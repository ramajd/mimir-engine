use std::time::{Duration, Instant};

use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;

const TARGET_FPS: f64 = 60.0;

pub struct MimirApp {
    window: Option<Window>,
    last_frame_time: Instant,
    accumulated_time: Duration,
}

impl MimirApp {
    pub fn new() -> Self {
        Self {
            window: None,
            last_frame_time: Instant::now(),
            accumulated_time: Duration::ZERO,
        }
    }
}

impl ApplicationHandler for MimirApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let attributes = Window::default_attributes()
            .with_title("Mimir Game Engine")
            .with_inner_size(winit::dpi::LogicalSize::new(800, 600));
        self.window = Some(
            event_loop
                .create_window(attributes)
                .expect("failed to create window"),
        );
        self.last_frame_time = Instant::now();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
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
            WindowEvent::RedrawRequested => {
                // TODO: render graphics
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        let target_frame_duration = Duration::from_secs_f64(1.0 / TARGET_FPS);
        let now = Instant::now();
        let delta_time = now - self.last_frame_time;
        self.last_frame_time = now;
        self.accumulated_time += delta_time;

        if self.accumulated_time > target_frame_duration {
            // TODO: update game logic
            if let Some(window) = &self.window {
                window.request_redraw();
                self.accumulated_time -= target_frame_duration;
            }
        }
    }
}
