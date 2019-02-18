use crate::camera::Camera;
use nalgebra::Vector2;
use nphysics2d::world::World;
use opengl_graphics::GlGraphics;

mod circle;
mod rectangle;

pub use self::circle::Circle;
pub use self::rectangle::Rectangle;

pub enum Part {
    Circle(Circle),
    Rectangle(Rectangle),
}

impl Part {
    pub fn update(&mut self, world: &World<f64>) {
        match self {
            Part::Circle(circle) => circle.update(world),
            Part::Rectangle(rectangle) => rectangle.update(world),
        }
    }

    pub fn draw(&self, camera: &Camera, ctx: graphics::Context, gfx: &mut GlGraphics) {
        match self {
            Part::Circle(circle) => circle.draw(camera, ctx, gfx),
            Part::Rectangle(rectangle) => rectangle.draw(camera, ctx, gfx),
        }
    }

    // pub fn create(&mut self, world: &World<f64>) {
    //     match self {
    //         Part::Circle(circle) => circle.create(world),
    //         Part::Rectangle(rectangle) => rectangle.create(world),
    //     }
    // }

    // pub fn destroy(&mut self, world: &World<f64>) {
    //     match self {
    //         Part::Circle(circle) => circle.destroy(world),
    //         Part::Rectangle(rectangle) => rectangle.destroy(world),
    //     }
    // }

    pub fn world_position(&self, world: &World<f64>) -> Vector2<f64> {
        unimplemented!()
    }

    pub fn world_rotation(&self, world: &World<f64>) -> f64 {
        unimplemented!()
    }

    // returns the position when the game is not running
    pub fn position(&self) -> Vector2<f64> {
        unimplemented!()
    }

    // returns the rotation when the game is not running
    pub fn rotation(&self) -> f64 {
        unimplemented!()
    }
}
