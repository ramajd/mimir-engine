use wgpu::Queue;
use winit::event::KeyEvent;

use crate::input::InputState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    MainMenu,
    Playing,
    GameOver,
}

pub trait StateMachine {
    fn update(&mut self, dt: f32, input: &InputState) -> Option<AppState>;
    fn render(&mut self, queue: &Queue);
    fn handle_keyboard(&mut self, event: &KeyEvent) -> Option<AppState>;
}
