use crate::camera::Camera;
use graphics::color;
use graphics::{Colored, Context, Transformed};
use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Ball, Cuboid, ShapeHandle};
use nphysics2d::joint::{ConstraintHandle, MouseConstraint};
use nphysics2d::object::{BodyPartHandle, ColliderDesc, ColliderHandle, RigidBodyDesc};
use nphysics2d::world::World;
use opengl_graphics::GlGraphics;

#[derive(Copy, Clone)]
pub enum ShapeKind {
    Circle { radius: f64 },
    Rectangle { half_width: f64, half_height: f64 },
}

pub struct Shape {
    kind: ShapeKind,
    iso: Isometry2<f64>,
    world_iso: Isometry2<f64>,
    handle: Option<ColliderHandle>,
    color: [f32; 4],
}

impl Shape {
    pub fn create(&mut self, camera: &Camera, world: &mut World<f64>) {
        if self.handle.is_some() {
            log::trace!("This shape is already initialized.");
            return;
        }
        let shape_handle = match self.kind {
            ShapeKind::Circle { radius } => ShapeHandle::new(Ball::new(radius)),
            ShapeKind::Rectangle {
                half_width,
                half_height,
            } => ShapeHandle::new(Cuboid::new(Vector2::new(half_width, half_height))),
        };
        let collider = ColliderDesc::new(shape_handle)
            .translation(self.iso.translation.vector)
            .build(world);
        // RigidBodyDesc::new()
        //     .collider(&collider)
        //     .set_translation(camera.to_local(self.iso.translation.vector))
        //     .build(world);
        self.handle = Some(collider.handle());
    }

    pub fn destroy(&mut self, world: &mut World<f64>) {
        if self.handle.is_none() {
            log::trace!("This shape doesn't exist!");
            return;
        }
        let collider = self.handle.unwrap();
        world.remove_colliders(&[collider]);
        self.handle = None;
    }

    pub fn update(&mut self, world: &World<f64>) {
        if let Some(handle) = self.handle {
            self.world_iso = world.collider(handle).unwrap().position().clone();
        }
    }

    pub fn draw(&self, camera: &Camera, ctx: Context, gfx: &mut GlGraphics) {
        let position = if self.handle.is_some() {
            self.world_iso.translation.vector
        } else {
            self.iso.translation.vector
        };
        let position = camera.to_global(position);
        let rotation = if self.handle.is_some() {
            self.world_iso.rotation.angle()
        } else {
            self.iso.rotation.angle()
        };
        let xf = ctx
            .trans(position.x, position.y)
            .rot_rad(rotation)
            .zoom(camera.zoom())
            .transform;
        match self.kind {
            ShapeKind::Circle { radius } => {
                use graphics::ellipse::Border;
                graphics::Ellipse::new(self.color)
                    .border(Border {
                        color: self.color.shade(0.5),
                        radius: 0.1,
                    })
                    .resolution(16)
                    .draw(
                        [-radius, -radius, radius * 2.0, radius * 2.0],
                        &graphics::DrawState::default(),
                        xf,
                        gfx,
                    );
            }
            ShapeKind::Rectangle {
                half_width,
                half_height,
            } => {
                use graphics::rectangle::Border;
                graphics::Rectangle::new(self.color)
                    .border(Border {
                        color: self.color.shade(0.5),
                        radius: 0.1,
                    })
                    .draw(
                        [
                            -half_width,
                            -half_height,
                            half_width * 2.0,
                            half_height * 2.0,
                        ],
                        &graphics::DrawState::default(),
                        xf,
                        gfx,
                    );
            }
        }
    }
}

pub struct ShapeBuilder {
    kind: ShapeKind,
    position: Vector2<f64>,
    rotation: f64,
    color: [f32; 4],
}

impl ShapeBuilder {
    pub fn circle(radius: f64) -> Self {
        ShapeBuilder {
            kind: ShapeKind::Circle { radius },
            position: nalgebra::zero(),
            rotation: 0.0,
            color: color::WHITE,
        }
    }

    pub fn rectangle(half_width: f64, half_height: f64) -> Self {
        ShapeBuilder {
            kind: ShapeKind::Rectangle {
                half_width,
                half_height,
            },
            position: nalgebra::zero(),
            rotation: 0.0,
            color: color::WHITE,
        }
    }

    pub fn position(&mut self, position: Vector2<f64>) -> &mut Self {
        self.position = position;
        self
    }

    pub fn rotation(&mut self, rotation: f64) -> &mut Self {
        self.rotation = rotation;
        self
    }

    pub fn color(&mut self, color: [f32; 4]) -> &mut Self {
        self.color = color;
        self
    }

    pub fn build(&self) -> Shape {
        Shape {
            kind: self.kind,
            iso: Isometry2::new(self.position, self.rotation),
            world_iso: Isometry2::identity(),
            handle: None,
            color: self.color,
        }
    }
}
