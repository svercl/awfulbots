use crate::camera::Camera;
use graphics::{Graphics, Transformed};
use nalgebra::Vector2;
use nphysics2d::object::ColliderHandle;
use nphysics2d::world::World;

pub struct Rectangle {
    position: Vector2<f64>,
    rotation: f64,
    handle: ColliderHandle,
    shape: graphics::Rectangle,
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(handle: ColliderHandle, world: &World<f64>, width: f64, height: f64) -> Self {
        let iso = world.collider(handle).unwrap().position();
        let shape = graphics::Rectangle::new([rand::random(), rand::random(), rand::random(), 1.0]);
        Rectangle {
            position: iso.translation.vector,
            rotation: iso.rotation.angle(),
            handle,
            shape,
            width,
            height,
        }
    }

    pub fn update(&mut self, world: &World<f64>) {
        let collider = world.collider(self.handle).unwrap();
        let iso = collider.position();
        self.position = iso.translation.vector;
        self.rotation = iso.rotation.angle();
    }

    pub fn draw<G>(&self, camera: &Camera, c: graphics::Context, g: &mut G)
    where
        G: Graphics,
    {
        let position = camera.to_global(&self.position);
        self.shape.draw(
            [
                -self.width,
                -self.height,
                self.width * 2.0,
                self.height * 2.0,
            ],
            &graphics::DrawState::default(),
            c.trans(position.x, position.y)
                .rot_rad(self.rotation)
                .zoom(camera.zoom())
                .transform,
            g,
        );
    }
}
