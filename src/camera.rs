use nalgebra::Vector2;

// camera holds information about the current viewport, and can translate from screen space to world space
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    // the top-left location of the camera
    position: Vector2<f64>,
    // current zoom
    zoom: f64,
    // the size of the viewport, usually this is the same size as the window
    size: Vector2<f64>,
}

impl Camera {
    // new creates a new camera with the specified width and height
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

    pub fn size(&self) -> &Vector2<f64> {
        &self.size
    }

    // sets the current size of the camera, useful when the window resizes
    pub fn set_size(&mut self, width: f64, height: f64) {
        self.size.x = width;
        self.size.y = height;
    }

    // moves by the specified amount
    pub fn trans(&mut self, amount: &Vector2<f64>) {
        self.position += amount;
    }

    // converts from global (screen) to world (local) space
    pub fn to_local(&self, global: Vector2<f64>) -> Vector2<f64> {
        self.position + (global - self.size / 2.0) / self.zoom
    }

    // converts from local (world) to global (screen) space
    pub fn to_global(&self, local: Vector2<f64>) -> Vector2<f64> {
        self.zoom * (local - self.position) + self.size / 2.0
    }
}
