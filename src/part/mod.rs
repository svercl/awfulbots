use crate::camera::Camera;
use graphics::Graphics;
use nphysics2d::world::World;

pub mod circle;
pub mod rectangle;

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

    pub fn draw<G>(&self, camera: &Camera, c: graphics::Context, g: &mut G)
    where
        G: Graphics,
    {
        match self {
            Part::Circle(circle) => circle.draw(camera, c, g),
            Part::Rectangle(rectangle) => rectangle.draw(camera, c, g),
        }
    }
}
