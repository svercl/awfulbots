use crate::camera::Camera;
use graphics::{Graphics, Transformed};
use nalgebra::Vector2;
use nphysics2d::object::ColliderHandle;
use nphysics2d::world::World;
use opengl_graphics::GlGraphics;

pub struct Circle {
    position: Vector2<f64>,
    rotation: f64,
    handle: ColliderHandle,
    shape: graphics::Ellipse,
    radius: f64,
}

impl Circle {
    pub fn new(handle: ColliderHandle, world: &World<f64>, radius: f64) -> Self {
        log::info!("Creating with radius of: {}", radius);

        let color = [rand::random(), rand::random(), rand::random(), 1.0];
        let shape = graphics::Ellipse::new(color)
            .border(Border {
                color: color.shade(0.5),
                radius: 0.1,
            })
            .resolution(16);
        Circle {
            position: iso.translation.vector,
            rotation: iso.rotation.angle(),
            handle,
            shape,
            radius,
        }
    }

    pub fn update(&mut self, world: &World<f64>) {
        let collider = world.collider(self.handle).unwrap();
        let iso = collider.position();
        self.position = iso.translation.vector;
        self.rotation = iso.rotation.angle();
    }

    pub fn draw(&self, camera: &Camera, ctx: graphics::Context, gfx: &mut GlGraphics) {
        let position = camera.to_global(&self.iso.translation.vector);
        self.shape.draw(
            [
                -self.radius,
                -self.radius,
                self.radius * 2.0,
                self.radius * 2.0,
            ],
            &graphics::DrawState::default(),
            ctx.trans(position.x, position.y)
                .rot_rad(self.rotation)
                .zoom(camera.zoom())
                .transform,
            gfx,
        );
    }
}
