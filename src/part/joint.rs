use crate::camera::Camera;
use crate::part::Shape;
use graphics::{Colored, Context, Transformed};
use nalgebra::{Point2, Vector2};
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
    anchor1: Point2<f64>,
    anchor2: Point2<f64>,
    axis: Vector2<f64>,
}

impl Joint {
    pub(super) fn create(&mut self, world: &mut World<f64>) {
        match self.kind {
            JointKind::Prismatic => {
                use nalgebra::Unit;
                use nphysics2d::joint::PrismaticConstraint;
                let body_part1 = world
                    .body(self.shapes.shape1.body_handle.unwrap())
                    .unwrap()
                    .part(0)
                    .unwrap()
                    .part_handle();
                let body_part2 = world
                    .body(self.shapes.shape2.body_handle.unwrap())
                    .unwrap()
                    .part(0)
                    .unwrap()
                    .part_handle();
                let joint = PrismaticConstraint::new(
                    body_part1,
                    body_part2,
                    self.anchor1,
                    Unit::new_normalize(self.axis),
                    self.anchor2,
                );
                self.handle = Some(world.add_constraint(joint));
            }
            _ => {}
        }
    }

    pub(super) fn destroy(&mut self, world: &mut World<f64>) {
        world.remove_constraint(self.handle.unwrap());
    }
}

pub struct JointBuilder {
    kind: JointKind,
    shapes: Shapes,
    anchor1: Point2<f64>,
    anchor2: Point2<f64>,
    axis: Vector2<f64>,
}

impl JointBuilder {
    pub fn fixed(shape1: Shape, shape2: Shape) -> Self {
        JointBuilder {
            kind: JointKind::Fixed,
            shapes: Shapes { shape1, shape2 },
            anchor1: Point2::origin(),
            anchor2: Point2::origin(),
            axis: nalgebra::zero(),
        }
    }

    pub fn prismatic(shape1: Shape, shape2: Shape) -> Self {
        JointBuilder {
            kind: JointKind::Prismatic,
            shapes: Shapes { shape1, shape2 },
            anchor1: Point2::origin(),
            anchor2: Point2::origin(),
            axis: nalgebra::zero(),
        }
    }

    pub fn anchor1(&mut self, anchor1: Point2<f64>) -> &mut Self {
        self.anchor1 = anchor1;
        self
    }

    pub fn anchor2(&mut self, anchor2: Point2<f64>) -> &mut Self {
        self.anchor2 = anchor2;
        self
    }

    pub fn axis(&mut self, axis: Vector2<f64>) -> &mut Self {
        self.axis = axis;
        self
    }

    pub fn build(&mut self) -> Joint {
        Joint {
            kind: self.kind,
            handle: None,
            shapes: self.shapes,
            anchor1: self.anchor1,
            anchor2: self.anchor2,
            axis: self.axis,
        }
    }
}
