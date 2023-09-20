// 2D camera
pub struct Camera2D {
    position: [f32; 2],
    zoom: f32,
}

impl Camera2D {
    fn new() -> Camera2D {
        Camera2D {
            position: [0.0, 0.0],
            zoom: 1.0,
        }
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.position = [x, y];
    }

    fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }
}