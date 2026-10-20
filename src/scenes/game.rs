use hecs::Entity;
use winit::{
    event::ElementState,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::{
    camera::{self, Camera2D},
    components::{Collider2D, PlayerTag, Transform},
    state::{AppState, StateMachine},
};

pub struct GamePlayState {
    pub world: hecs::World,
    pub camera: Camera2D,
}

impl GamePlayState {
    pub fn new() -> Self {
        let mut world = hecs::World::new();
        let camera = Camera2D::new(glam::Vec2::ZERO);

        let quad_size = 0.6;

        // Player Entity
        world.spawn((
            PlayerTag,
            Transform {
                position: glam::Vec3::new(-1.0, 0.0, 0.0),
                rotation: 0.0,
                scale: glam::Vec3::ONE,
            },
            Collider2D::new(quad_size, quad_size),
        ));

        world.spawn((
            Transform {
                position: glam::Vec3::new(1.0, 0.0, 0.0),
                rotation: 0.0,
                scale: glam::Vec3::ONE,
            },
            Collider2D::new(quad_size, quad_size),
        ));

        Self { world, camera }
    }
}

impl StateMachine for GamePlayState {
    fn update(
        &mut self,
        dt: f32,
        input: &crate::input::InputState,
    ) -> Option<crate::state::AppState> {
        let speed = 3.0;
        let mut move_dir = glam::Vec3::ZERO;
        if input.up {
            move_dir.y += 1.0;
        }
        if input.down {
            move_dir.y -= 1.0;
        }
        if input.left {
            move_dir.x -= 1.0;
        }
        if input.right {
            move_dir.x += 1.0;
        }

        let velocity = move_dir.normalize_or_zero() * speed * dt;

        // Simple Movement and Camera update
        for (_, _, transform) in &mut self.world.query::<(Entity, &PlayerTag, &mut Transform)>() {
            transform.position += velocity;
            self.camera.position = self
                .camera
                .position
                .lerp(transform.position.truncate(), 5.0 * dt);
        }

        for (entity, transform) in &mut self.world.query::<(Entity, &mut Transform)>() {
            if self.world.get::<&PlayerTag>(entity).is_err() {
                transform.rotation -= 2.0 * dt;
            }
        }
        None
    }

    fn render(&mut self, _queue: &wgpu::Queue) {
        // rendering handled by engine
    }

    fn handle_keyboard(
        &mut self,
        event: &winit::event::KeyEvent,
    ) -> Option<crate::state::AppState> {
        if event.state == ElementState::Pressed {
            if let PhysicalKey::Code(KeyCode::Escape) = event.physical_key {
                println!("Returning to Main Menu...");
                return Some(AppState::MainMenu);
            }
        }
        None
    }
}
