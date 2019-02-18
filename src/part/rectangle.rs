use crate::camera::Camera;
use graphics::rectangle::Border;
use graphics::{Colored, Transformed};
use nalgebra::Isometry2;
use nphysics2d::object::ColliderHandle;
use nphysics2d::world::World;
use opengl_graphics::GlGraphics;

pub struct Rectangle {
    iso: Isometry2<f64>,
    handle: ColliderHandle,
    shape: graphics::Rectangle,
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(handle: ColliderHandle, world: &World<f64>, width: f64, height: f64) -> Self {
        log::info!("Creating with dimensions: {}x{}", width, height);

        let iso = world.collider(handle).unwrap().position().clone();
        let color = [rand::random(), rand::random(), rand::random(), 1.0];
        let shape = graphics::Rectangle::new(color).border(Border {
            color: color.shade(0.5),
            radius: 0.1,
        });

        Rectangle {
            iso,
            handle,
            shape,
            width,
            height,
        }
    }

    pub fn update(&mut self, world: &World<f64>) {
        self.iso = world.collider(self.handle).unwrap().position().clone()
    }

    pub fn draw(&self, camera: &Camera, ctx: graphics::Context, gfx: &mut GlGraphics) {
        let position = camera.to_global(&self.iso.translation.vector);
        self.shape.draw(
            [
                -self.width,
                -self.height,
                self.width * 2.0,
                self.height * 2.0,
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
