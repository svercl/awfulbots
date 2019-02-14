use crate::camera::Camera;
use crate::part;
use graphics::Transformed;
use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Ball, Cuboid, ShapeHandle};
use ncollide2d::world::CollisionGroups;
use nphysics2d::material::{BasicMaterial, MaterialHandle};
use nphysics2d::object::{ColliderDesc, ColliderHandle, RigidBodyDesc};
use nphysics2d::world::World;
use opengl_graphics::GlGraphics;
use piston::input::Key;

pub struct State {
    camera: Camera,
    world: World<f64>,
    parts: Vec<part::Part>,
}

impl State {
    pub fn new(camera: Camera) -> Self {
        let mut world = World::new();
        world.set_gravity(Vector2::new(0.0, -9.81));

        let ground_size = 25.0;
        let ground_shape = ShapeHandle::new(Cuboid::new(Vector2::new(ground_size, 1.0)));
        ColliderDesc::new(ground_shape)
            .translation(-Vector2::y())
            .build(&mut world);

        let num = 25;
        let rad = 1.0;

        let cuboid = ShapeHandle::new(Ball::new(rad));
        let collider_desc = ColliderDesc::new(cuboid).density(1.0);
        let mut rb_desc = RigidBodyDesc::new().collider(&collider_desc);

        let shift = (rad + collider_desc.get_margin()) * 2.0 + 0.002;
        let centerx = shift * (num as f64) / 2.0;
        let centery = shift / 2.0;

        for i in 0usize..num {
            for j in 0usize..num {
                let x = i as f64 * shift - centerx;
                let y = j as f64 * shift + centery;

                rb_desc
                    .set_translation(Vector2::new(x, y))
                    .build(&mut world);
            }
        }

        let parts = world
            .colliders()
            .filter_map(|collider| {
                let shape = collider.shape().as_ref();
                let margin = collider.margin();
                if let Some(shape) = shape.as_shape::<Ball<f64>>() {
                    Some(part::Part::Circle(part::Circle::new(
                        collider.handle(),
                        &world,
                        shape.radius() + margin,
                    )))
                } else if let Some(shape) = shape.as_shape::<Cuboid<f64>>() {
                    let he = shape.half_extents();
                    Some(part::Part::Rectangle(part::Rectangle::new(
                        collider.handle(),
                        &world,
                        he.x * 2.0 + margin,
                        he.y * 2.0 + margin,
                    )))
                } else {
                    println!("Unknown shape");
                    None
                }
            })
            .collect::<Vec<_>>();

        State {
            camera,
            world,
            parts,
        }
    }

    pub fn update(&mut self) {
        self.world.step();

        for part in &mut self.parts {
            part.update(&self.world);
        }
    }

    pub fn draw(&self, ctx: graphics::Context, gl: &mut GlGraphics) {
        for part in &self.parts {
            part.draw(&self.camera, ctx, gl);
        }
    }

    pub fn key(&mut self, key: Key, pressed: bool) {
        match key {
            Key::A | Key::Left if pressed => self.camera.trans(&Vector2::new(-10.0, 0.0)),
            Key::D | Key::Right if pressed => self.camera.trans(&Vector2::new(10.0, 0.0)),
            Key::W | Key::Up if pressed => self.camera.trans(&Vector2::new(0.0, 10.0)),
            Key::S | Key::Dollar if pressed => self.camera.trans(&Vector2::new(0.0, -10.0)),
            Key::R if pressed => self.camera.set_zoom(self.camera.zoom() * 4.0 / 3.0),
            Key::F if pressed => self.camera.set_zoom(self.camera.zoom() * 3.0 / 4.0),
            _ => {}
        }
    }
}
