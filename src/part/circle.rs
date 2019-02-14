use crate::camera::Camera;
use graphics::{Graphics, Transformed};
use nalgebra::Vector2;
use nphysics2d::object::ColliderHandle;
use nphysics2d::world::World;

pub struct Circle {
    position: Vector2<f64>,
    handle: ColliderHandle,
    shape: graphics::Ellipse,
    radius: f64,
}

impl Circle {
    pub fn new(handle: ColliderHandle, world: &World<f64>, radius: f64) -> Self {
        let position = {
            let iso = world.collider(handle).unwrap().position();
            iso.translation.vector
        };
        let shape = graphics::Ellipse::new([1.0, 1.0, 1.0, 1.0]).resolution(16);
        Circle {
            position,
            handle,
            shape,
            radius,
        }
    }

    pub fn update(&mut self, world: &World<f64>) {
        self.position = world
            .collider(self.handle)
            .unwrap()
            .position()
            .translation
            .vector;
    }

    pub fn draw<G: Graphics>(&self, camera: &Camera, c: graphics::Context, g: &mut G) {
        let position = camera.to_global(&self.position);
        self.shape.draw(
            [0.0, 0.0, self.radius, self.radius],
            &graphics::DrawState::default(),
            c.trans(position.x, position.y)
                .zoom(camera.zoom())
                .transform,
            g,
        );
    }
}
