use crate::camera::Camera;
use graphics::ellipse::Border;
use graphics::{Colored, Transformed};
use nalgebra::Isometry2;
use nphysics2d::object::ColliderHandle;
use nphysics2d::world::World;
use opengl_graphics::GlGraphics;

pub struct Circle {
    iso: Isometry2<f64>,
    handle: ColliderHandle,
    shape: graphics::Ellipse,
    radius: f64,
}

impl Circle {
    pub fn new(handle: ColliderHandle, world: &World<f64>, radius: f64) -> Self {
        log::info!("Creating with radius of: {}", radius);

        let iso = world.collider(handle).unwrap().position().clone();
        let color = [rand::random(), rand::random(), rand::random(), 1.0];
        let shape = graphics::Ellipse::new(color)
            .border(Border {
                color: color.shade(0.5),
                radius: 0.1,
            })
            .resolution(16);

        Circle {
            iso,
            handle,
            shape,
            radius,
        }
    }

    pub fn update(&mut self, world: &World<f64>) {
        self.iso = world.collider(self.handle).unwrap().position().clone();
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
                .rot_rad(self.iso.rotation.angle())
                .zoom(camera.zoom())
                .transform,
            gfx,
        );
    }
}
