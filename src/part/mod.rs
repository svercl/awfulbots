use nalgebra::Vector2;
use nphysics2d::world::World;
use std::fmt::Debug;

mod joint;
mod shape;

pub use self::joint::{Joint, JointBuilder, JointKind};
pub use self::shape::{Shape, ShapeBuilder, ShapeKind};

pub trait Part: Debug {
    fn update(&mut self, world: &World<f64>);
    fn create(&mut self, world: &mut World<f64>);
    fn destroy(&mut self, world: &mut World<f64>);
    fn is_point_inside(&self, point: Vector2<f64>) -> bool;

    fn as_shape(&self) -> Option<&Shape> {
        None
    }

    fn as_joint(&self) -> Option<&Joint> {
        None
    }
}
