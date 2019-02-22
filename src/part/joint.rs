use crate::camera::Camera;
use crate::part::Shape;
use graphics::{Colored, Context, Transformed};
use nphysics2d::joint::ConstraintHandle;
use nphysics2d::object::BodyHandle;
use nphysics2d::world::World;

#[derive(Clone, Copy)]
pub enum JointKind {
    Fixed,
    Prismatic,
    Revolute,
}

#[derive(Clone, Copy)]
struct Shapes {
    shape1: Shape,
    shape2: Shape,
}

pub struct Joint {
    kind: JointKind,
    handle: Option<ConstraintHandle>,
    shapes: Shapes,
}

impl Joint {
    pub(super) fn create(&mut self, world: &mut World<f64>) {}
    pub(super) fn destroy(&mut self, world: &mut World<f64>) {}
}

pub struct JointBuilder {
    kind: JointKind,
    shapes: Shapes,
}

impl JointBuilder {
    pub fn fixed(shape1: Shape, shape2: Shape) -> Self {
        JointBuilder {
            kind: JointKind::Fixed,
            shapes: Shapes { shape1, shape2 },
        }
    }

    pub fn build(&mut self) -> Joint {
        Joint {
            kind: self.kind,
            handle: None,
            shapes: self.shapes,
        }
    }
}
