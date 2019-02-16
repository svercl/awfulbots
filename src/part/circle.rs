use crate::camera::Camera;
use graphics::{Graphics, Transformed};
use nalgebra::Vector2;
use nphysics2d::object::ColliderHandle;
use nphysics2d::world::World;

pub struct Circle {
    position: Vector2<f64>,
    rotation: f64,
    handle: ColliderHandle,
    shape: graphics::Ellipse,
    radius: f64,
}

impl Circle {
    pub fn new(handle: ColliderHandle, world: &World<f64>, radius: f64) -> Self {
        log::info!("Creating `Circle` with radius of: {}", radius);

        let iso = world.collider(handle).unwrap().position();
        let shape = graphics::Ellipse::new([rand::random(), rand::random(), rand::random(), 1.0])
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

    pub fn draw<G: Graphics>(&self, camera: &Camera, ctx: graphics::Context, gfx: &mut G) {
        let position = camera.to_global(&self.position);
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
