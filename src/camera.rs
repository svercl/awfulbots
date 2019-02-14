use nalgebra::Vector2;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    position: Vector2<f64>,
    zoom: f64,
    size: Vector2<f64>,
}

impl Camera {
    pub fn new(width: f64, height: f64) -> Self {
        Camera {
            position: nalgebra::zero(),
            zoom: 30.0,
            size: Vector2::new(width, height),
        }
    }

    pub fn zoom(&self) -> f64 {
        self.zoom
    }

    pub fn set_zoom(&mut self, zoom: f64) {
        self.zoom = zoom;
    }

    pub fn set_size(&mut self, width: f64, height: f64) {
        self.size.x = width;
        self.size.y = height;
    }

    pub fn trans(&mut self, amount: &Vector2<f64>) {
        self.position += amount;
    }

    pub fn to_local(&self, global: &Vector2<f64>) -> Vector2<f64> {
        self.position + (global - self.size / 2.0) / self.zoom
    }

    pub fn to_global(&self, local: &Vector2<f64>) -> Vector2<f64> {
        self.zoom * (local - self.position) + self.size / 2.0
    }
}
