use crate::camera::Camera;
use graphics::{Colored, Graphics, Transformed};
use nalgebra::Vector2;
use nphysics2d::object::ColliderHandle;
use nphysics2d::world::World;
use opengl_graphics::GlGraphics;

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
        log::info!("Creating with dimensions: {}x{}", width, height);

        let iso = world.collider(handle).unwrap().position();
        let color = [rand::random(), rand::random(), rand::random(), 1.0];
        let shape = graphics::Rectangle::new(color).border(graphics::rectangle::Border {
            color: color.shade(0.5),
            radius: 0.1,
        });
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

    pub fn draw(&self, camera: &Camera, ctx: graphics::Context, gfx: &mut GlGraphics) {
        self.shape.draw(
            [
                -self.width,
                -self.height,
                self.width * 2.0,
                self.height * 2.0,
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
