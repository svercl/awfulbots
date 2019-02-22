use crate::camera::Camera;
use nphysics2d::world::World;
use opengl_graphics::GlGraphics;

mod shape;

pub use self::shape::{Shape, ShapeBuilder, ShapeKind};

pub enum Part {
    Shape(Shape),
}

impl Part {
    pub fn update(&mut self, world: &World<f64>) {
        match self {
            Part::Shape(shape) => shape.update(world),
        }
    }

    pub fn draw(&self, camera: &Camera, ctx: graphics::Context, gfx: &mut GlGraphics) {
        match self {
            Part::Shape(shape) => shape.draw(camera, ctx, gfx),
        }
    }

    pub fn create(&mut self, world: &mut World<f64>) {
        match self {
            Part::Shape(shape) => shape.create(world),
        }
    }

    pub fn destroy(&mut self, world: &mut World<f64>) {
        match self {
            Part::Shape(shape) => shape.destroy(world),
        }
    }

    pub fn set_selected(&mut self, selected: bool) {
        match self {
            Part::Shape(shape) => shape.selected = selected,
        }
    }
}
