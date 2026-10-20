use winit::{
    event::ElementState,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::state::{AppState, StateMachine};

pub struct MainMenuState;

impl MainMenuState {
    pub fn new() -> Self {
        println!("Entering Main Menu State. Press Enter to start playing");
        Self
    }
}

impl StateMachine for MainMenuState {
    fn update(
        &mut self,
        _dt: f32,
        _input: &crate::input::InputState,
    ) -> Option<crate::state::AppState> {
        None
    }

    fn render(&mut self, _queue: &wgpu::Queue) {
        // UI render will be handled later
    }

    fn handle_keyboard(
        &mut self,
        event: &winit::event::KeyEvent,
    ) -> Option<crate::state::AppState> {
        if event.state == ElementState::Pressed {
            if let PhysicalKey::Code(KeyCode::Enter) = event.physical_key {
                println!("Transitioning to Playing State ...");
                return Some(AppState::Playing);
            }
        }
        None
    }
}
