use crate::camera::Camera;
use nphysics2d::world::World;
use opengl_graphics::GlGraphics;

mod joint;
mod shape;

pub use self::joint::{Joint, JointBuilder, JointKind};
pub use self::shape::{Shape, ShapeBuilder, ShapeKind};

pub enum Part {
    Joint(Joint),
    Shape(Shape),
}

impl Part {
    pub fn update(&mut self, world: &World<f64>) {
        match self {
            Part::Shape(shape) => shape.update(world),
            _ => {}
        }
    }

    pub fn create(&mut self, world: &mut World<f64>) {
        match self {
            Part::Shape(shape) => shape.create(world),
            Part::Joint(joint) => joint.create(world),
        }
    }

    pub fn destroy(&mut self, world: &mut World<f64>) {
        match self {
            Part::Shape(shape) => shape.destroy(world),
            Part::Joint(joint) => joint.destroy(world),
        }
    }

    pub fn is_point_inside(&self, point: nalgebra::Point2<f64>) -> bool {
        match self {
            Part::Shape(shape) => shape.is_point_inside(point),
            _ => false,
        }
    }

    pub fn set_selected(&mut self, selected: bool) {
        match self {
            Part::Shape(shape) => shape.selected = selected,
            _ => {}
        }
    }
}
