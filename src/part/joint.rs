use crate::camera::Camera;
use crate::part::{Part, Shape};
use graphics::{Colored, Context, Transformed};
use nalgebra::{Point2, Vector2};
use nphysics2d::joint::ConstraintHandle;
use nphysics2d::object::BodyHandle;
use nphysics2d::world::World;

#[derive(Clone, Copy, Debug)]
pub enum JointKind {
    Fixed,
    Prismatic,
    Revolute,
}

#[derive(Debug)]
struct Shapes {
    shape1: Shape,
    shape2: Shape,
}

#[derive(Debug)]
pub struct Joint {
    kind: JointKind,
    handle: Option<ConstraintHandle>,
    shapes: Shapes,
    anchor1: Point2<f64>,
    anchor2: Point2<f64>,
    axis: Vector2<f64>,
}

impl Joint {
    pub fn anchor1(&self) -> Point2<f64> {
        self.anchor1
    }

    pub fn anchor2(&self) -> Point2<f64> {
        self.anchor2
    }

    pub fn kind(&self) -> JointKind {
        self.kind
    }
}

impl Part for Joint {
    fn update(&mut self, world: &World<f64>) {}

    fn create(&mut self, world: &mut World<f64>) {
        match self.kind {
            JointKind::Prismatic => {
                use nalgebra::Unit;
                use nphysics2d::joint::PrismaticConstraint;
                let body_part1 = world
                    .body(self.shapes.shape1.body_handle.expect("No body on shape1"))
                    .expect("Body for shape1 doesn't exist")
                    .part(0)
                    .expect("Part for shape1 doesn't exist")
                    .part_handle();
                let body_part2 = world
                    .body(self.shapes.shape2.body_handle.expect("No body on shape2"))
                    .expect("Body for shape2 doesn't exist")
                    .part(0)
                    .expect("Part for shape2 doesn't exist")
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

    fn destroy(&mut self, world: &mut World<f64>) {
        if let Some(handle) = self.handle {
            world.remove_constraint(handle);
        }
    }

    fn is_point_inside(&self, point: Vector2<f64>) -> bool {
        false
    }

    fn as_joint(&self) -> Option<&Joint> {
        Some(self)
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

    pub fn prismatic(shape1: Shape, shape2: &Shape) -> Self {
        // JointBuilder {
        //     kind: JointKind::Prismatic,
        //     shapes: Shapes {
        //         shape1: shape1.clone(),
        //         shape2: shape2.clone(),
        //     },
        //     anchor1: Point2::origin(),
        //     anchor2: Point2::origin(),
        //     axis: nalgebra::zero(),
        // }
        unimplemented!()
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
        // Joint {
        //     kind: self.kind,
        //     handle: None,
        //     shapes: self.shapes,
        //     anchor1: self.anchor1,
        //     anchor2: self.anchor2,
        //     axis: self.axis,
        // }
        unimplemented!()
    }
}
