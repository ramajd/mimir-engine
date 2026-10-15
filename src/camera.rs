pub struct Camera2D {
    pub position: glam::Vec2,
    pub zoom: f32,
}

impl Camera2D {
    pub fn new(position: glam::Vec2) -> Self {
        Self {
            position,
            zoom: 1.0,
        }
    }

    pub fn build_view_projection_matrix(&self, width: u32, height: u32) -> glam::Mat4 {
        let aspect = width as f32 / height as f32;
        let scale = 5.0 / self.zoom;

        let left = -aspect * scale + self.position.x;
        let right = aspect * scale + self.position.x;
        let bottom = -scale + self.position.y;
        let top = scale + self.position.y;
        let projection =
            glam::camera::rh::proj::directx::orthographic(left, right, bottom, top, -1.0, 1.0);
        projection
    }
}
