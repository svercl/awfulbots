use crate::camera::Camera;
use graphics::color;
use graphics::{Colored, Context, Transformed};
use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Ball, Cuboid, ShapeHandle};
use nphysics2d::object::{BodyHandle, BodyStatus, ColliderDesc, RigidBodyDesc};
use nphysics2d::world::World;
use opengl_graphics::GlGraphics;

#[derive(Clone, Copy, Debug)]
pub enum ShapeKind {
    Circle {
        radius: f64,
    },
    Rectangle {
        half_width: f64,
        half_height: f64,
    },
    Triangle {
        p1: Vector2<f64>,
        p2: Vector2<f64>,
        p3: Vector2<f64>,
    },
}

#[derive(Clone, Copy)]
pub struct Shape {
    kind: ShapeKind,
    iso: Isometry2<f64>,
    world_iso: Isometry2<f64>,
    pub(super) body_handle: Option<BodyHandle>,
    color: [f32; 4],
    ground: bool,
    // cool rust 2018 thingy
    pub(super) selected: bool,
}

impl Shape {
    pub fn iso(&self) -> Isometry2<f64> {
        self.iso
    }

    pub fn world_iso(&self) -> Isometry2<f64> {
        self.world_iso
    }

    pub fn kind(&self) -> ShapeKind {
        self.kind
    }

    pub fn color(&self) -> [f32; 4] {
        self.color
    }

    pub(super) fn create(&mut self, world: &mut World<f64>) {
        let shape_handle = match self.kind {
            ShapeKind::Circle { radius } => ShapeHandle::new(Ball::new(radius)),
            ShapeKind::Rectangle {
                half_width,
                half_height,
            } => ShapeHandle::new(Cuboid::new(Vector2::new(half_width, half_height))),
            ShapeKind::Triangle { p1, p2, p3 } => {
                use nalgebra::Point2;
                use ncollide2d::shape::ConvexPolygon;
                let points = &[p1, p2, p3]
                    .iter()
                    .map(|p| Point2::new(p.x, p.y))
                    .collect::<Vec<_>>();
                ShapeHandle::new(ConvexPolygon::try_from_points(&points).expect("FIXME"))
            }
        };
        let collider = ColliderDesc::new(shape_handle).density(1.0);
        let rigid_body = RigidBodyDesc::new()
            .collider(&collider)
            .status(if self.ground {
                BodyStatus::Static
            } else {
                BodyStatus::Dynamic // default
            })
            .position(self.iso)
            .build(world);
        self.body_handle = Some(rigid_body.handle());
    }

    pub(super) fn destroy(&mut self, world: &mut World<f64>) {
        if let Some(handle) = self.body_handle {
            world.remove_bodies(&[handle]);
            self.body_handle = None;
        }
    }

    pub(super) fn update(&mut self, world: &World<f64>) {
        if let Some(handle) = self.body_handle {
            if let Some(body) = world.rigid_body(handle) {
                self.world_iso = *body.position();
            }
        }
    }

    pub(super) fn is_point_inside(&self, point: nalgebra::Point2<f64>) -> bool {
        match self.kind {
            ShapeKind::Circle { radius } => {
                let pos = self.iso.translation.vector;
                point.x >= pos.x - radius
                    && point.x <= pos.x + radius
                    && point.y >= pos.y - radius
                    && point.x <= pos.y + radius
            }
            ShapeKind::Rectangle {
                half_width,
                half_height,
            } => {
                log::trace!("Checking if point is inside rectangle is not implemented yet.");
                false
            }
            ShapeKind::Triangle { p1, p2, p3 } => {
                log::trace!("Checking if point is inside triangle is not implemented yet.");
                false
            }
        }
    }
}

pub struct ShapeBuilder {
    kind: ShapeKind,
    position: Vector2<f64>,
    rotation: f64,
    color: [f32; 4],
    ground: bool,
    selected: bool,
}

impl ShapeBuilder {
    pub fn circle(radius: f64) -> Self {
        ShapeBuilder {
            kind: ShapeKind::Circle { radius },
            position: nalgebra::zero(),
            rotation: 0.0,
            color: color::WHITE,
            ground: false,
            selected: false,
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
            ground: false,
            selected: false,
        }
    }

    pub fn triangle(p1: Vector2<f64>, p2: Vector2<f64>, p3: Vector2<f64>) -> Self {
        ShapeBuilder {
            kind: ShapeKind::Triangle { p1, p2, p3 },
            position: nalgebra::zero(),
            rotation: 0.0,
            color: color::WHITE,
            ground: false,
            selected: false,
        }
    }

    pub fn position(&mut self, position: Vector2<f64>) -> &mut Self {
        self.position = position;
        self
    }

    pub fn position_p(&mut self, position: nalgebra::Point2<f64>) -> &mut Self {
        self.position.x = position.x;
        self.position.y = position.y;
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

    pub fn ground(&mut self, ground: bool) -> &mut Self {
        self.ground = ground;
        self
    }

    pub fn selected(&mut self, selected: bool) -> &mut Self {
        self.selected = selected;
        self
    }

    pub fn build(&self) -> Shape {
        Shape {
            kind: self.kind,
            iso: Isometry2::new(self.position, self.rotation),
            world_iso: Isometry2::identity(),
            body_handle: None,
            color: self.color,
            ground: self.ground,
            selected: self.selected,
        }
    }
}
