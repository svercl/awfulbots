use crate::camera::Camera;
use crate::part;
use crate::util;
use graphics::Transformed;
use nalgebra::{Isometry2, Point2, Vector2};
use ncollide2d::shape::{Ball, Cuboid, ShapeHandle};
use ncollide2d::world::CollisionGroups;
use nphysics2d::joint::{ConstraintHandle, MouseConstraint};
use nphysics2d::material::{BasicMaterial, MaterialHandle};
use nphysics2d::object::{BodyPartHandle, ColliderDesc, ColliderHandle, RigidBodyDesc};
use nphysics2d::world::World;
use opengl_graphics::GlGraphics;
use piston::input::{Key, MouseButton};

pub struct State {
    camera: Camera,
    world: World<f64>,
    parts: Vec<part::Part>,
    mouse_position: Vector2<f64>,
    mouse_position_world: Point2<f64>,
    grabbed_object: Option<BodyPartHandle>,
    grabbed_object_constraint: Option<ConstraintHandle>,
}

impl State {
    pub fn new(camera: Camera) -> Self {
        let mut world = World::new();
        world.set_gravity(Vector2::new(0.0, -30.0));

        let ground_size = 25.0;
        let ground_shape = ShapeHandle::new(Cuboid::new(Vector2::new(ground_size, 1.0)));
        ColliderDesc::new(ground_shape)
            .translation(-Vector2::y())
            .build(&mut world);

        let num = 10;
        let rad = 1.0;

        let cuboid = ShapeHandle::new(Cuboid::new(Vector2::repeat(rad)));
        let collider_desc = ColliderDesc::new(cuboid).density(1.0);
        let mut rb_desc = RigidBodyDesc::new().collider(&collider_desc);

        let shift = (rad + collider_desc.get_margin()) * 2.0;
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
                        he.x + margin,
                        he.y + margin,
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
            mouse_position: nalgebra::zero(),
            mouse_position_world: Point2::origin(),
            grabbed_object: None,
            grabbed_object_constraint: None,
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
            Key::Plus | Key::NumPadPlus if pressed => {
                self.camera.set_zoom(self.camera.zoom() * 4.0 / 3.0)
            }
            Key::Minus | Key::NumPadMinus if pressed => {
                self.camera.set_zoom(self.camera.zoom() * 3.0 / 4.0)
            }
            _ => {}
        }
    }

    pub fn mouse(&mut self, x: f64, y: f64) {
        self.mouse_position.x = x;
        self.mouse_position.y = y;
        let point = self.camera.to_local(&self.mouse_position);
        self.mouse_position_world.x = point.x;
        self.mouse_position_world.y = point.y;

        if self.grabbed_object.is_some() {
            let joint = self.grabbed_object_constraint.unwrap();
            let joint = self
                .world
                .constraint_mut(joint)
                .downcast_mut::<MouseConstraint<f64>>()
                .unwrap();
            joint.set_anchor_1(self.mouse_position_world);
        }
    }

    pub fn mouse_button(&mut self, button: MouseButton, pressed: bool) {
        match button {
            MouseButton::Left if pressed => {
                if let Some(body) = util::get_body_at_mouse(&self.world, &self.mouse_position_world)
                {
                    self.grabbed_object = Some(body);
                    if let Some(joint) = self.grabbed_object_constraint {
                        let _ = self.world.remove_constraint(joint);
                    }

                    let body_pos = self
                        .world
                        .body(body.0)
                        .unwrap()
                        .part(body.1)
                        .unwrap()
                        .position();
                    let body_mass = self
                        .world
                        .body(body.0)
                        .unwrap()
                        .part(body.1)
                        .unwrap()
                        .local_inertia()
                        .mass();
                    let anchor1 = self.mouse_position_world;
                    let anchor2 = body_pos.inverse() * anchor1;
                    let joint = MouseConstraint::new(
                        BodyPartHandle::ground(),
                        body,
                        anchor1,
                        anchor2,
                        300.0 * body_mass,
                    );
                    self.grabbed_object_constraint = Some(self.world.add_constraint(joint));
                }
            }
            MouseButton::Left if !pressed => {
                if let Some(joint) = self.grabbed_object_constraint {
                    let _ = self.world.remove_constraint(joint);
                }
                self.grabbed_object = None;
                self.grabbed_object_constraint = None;
            }
            _ => {}
        }
    }
}
