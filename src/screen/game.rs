use crate::action::ActionKind;
use crate::camera::Camera;
use crate::gui::GuiEvent;
use crate::limits;
use crate::part;
use crate::screen::Screen;
use crate::util;
use graphics::Transformed;
use nalgebra::{Point2, Vector2};
use nphysics2d::joint::{ConstraintHandle, MouseConstraint};
use nphysics2d::object::BodyPartHandle;
use nphysics2d::world::World;
use opengl_graphics::{GlGraphics, GlyphCache};
use piston::input::{Key, MouseButton};
use std::sync::mpsc;

pub struct GameScreen {
    camera: Camera,
    world: World<f64>,
    parts: Vec<part::Part>,
    mouse_position: Vector2<f64>,
    mouse_position_world: Point2<f64>,
    grabbed_object: Option<BodyPartHandle>,
    grabbed_object_constraint: Option<ConstraintHandle>,
    middle_mouse_down: bool,
    running: bool,
    gui_rx: mpsc::Receiver<GuiEvent>,
    current_action: ActionKind,
    current_action_step: u8,
    first_click: Vector2<f64>,
    first_click_world: Point2<f64>,
    second_click: Vector2<f64>,
    second_click_world: Point2<f64>,
}

impl GameScreen {
    pub fn new(camera: Camera, gui_rx: mpsc::Receiver<GuiEvent>) -> Self {
        let mut world = World::new();
        world.set_gravity(Vector2::new(0.0, 30.0));

        let mut parts = Vec::new();

        parts.push(part::Part::Shape(
            part::ShapeBuilder::rectangle(25.0, 1.0)
                .position(-Vector2::y())
                .ground(true)
                .build(),
        ));

        let rad = 1.0;

        let width = 5;
        let height = 10;
        let shift = 0.5 * rad;
        let centerx = shift * (width as f64) / 2.0;

        for i in 0usize..width {
            for j in 0usize..height {
                let fj = j as f64;
                let fi = i as f64;
                let x = fj * 5.0 * rad - centerx;
                let y = -fi * 5.0 * rad - 0.04 - rad;

                parts.push(part::Part::Shape(
                    part::ShapeBuilder::circle(rad)
                        .position(Vector2::new(x, y))
                        .color([rand::random(), rand::random(), rand::random(), 1.0])
                        .build(),
                ));
            }
        }

        GameScreen {
            camera,
            world,
            parts,
            mouse_position: nalgebra::zero(),
            mouse_position_world: Point2::origin(),
            grabbed_object: None,
            grabbed_object_constraint: None,
            middle_mouse_down: false,
            running: false,
            gui_rx,
            current_action: ActionKind::None,
            current_action_step: 0,
            first_click: nalgebra::zero(),
            first_click_world: Point2::origin(),
            second_click: nalgebra::zero(),
            second_click_world: Point2::origin(),
        }
    }

    fn zoom_in(&mut self) {
        self.camera.set_zoom(self.camera.zoom() * 4.0 / 3.0)
    }

    fn zoom_out(&mut self) {
        self.camera.set_zoom(self.camera.zoom() * 3.0 / 4.0)
    }
}

impl Screen for GameScreen {
    fn update(&mut self, dt: f64) {
        self.world.step();

        for part in &mut self.parts {
            part.update(&self.world);
        }

        if let Some(e) = self.gui_rx.try_recv().ok() {
            match e {
                GuiEvent::CircleClicked if !self.running => {
                    self.current_action = ActionKind::CreatingCircle
                }
                GuiEvent::RectangleClicked if !self.running => {
                    self.current_action = ActionKind::CreatingRectangle
                }
                GuiEvent::TriangleClicked if !self.running => {
                    self.current_action = ActionKind::CreatingTriangle;
                }
                GuiEvent::PlayClicked => {
                    self.running = !self.running;
                    for part in &mut self.parts {
                        if self.running {
                            part.create(&mut self.world);
                        } else {
                            part.destroy(&mut self.world);
                        }
                    }
                }
                f => log::trace!("Got event from GUI: {:?}", f),
            }

            if self.current_action != ActionKind::None {
                // reset the action step and first click
                self.current_action_step = 0;
                self.first_click = nalgebra::zero();
                self.first_click_world = Point2::origin();
                self.second_click = nalgebra::zero();
                self.second_click_world = Point2::origin();
                log::trace!("Starting action: {:?}", self.current_action,);
            }
        }
    }

    fn draw(&self, ctx: graphics::Context, gfx: &mut GlGraphics, glyphs: &mut GlyphCache<'static>) {
        for part in &self.parts {
            part.draw(&self.camera, ctx, gfx);
        }

        for (_, _, _, manifold) in self.world.collider_world().contact_pairs(true) {
            for c in manifold.contacts() {
                let color = if c.contact.depth < 0.0 {
                    [0.0, 0.0, 1.0, 1.0]
                } else {
                    [1.0, 0.0, 0.0, 1.0]
                };
                let world1 = self
                    .camera
                    .to_global(Vector2::new(c.contact.world1.x, c.contact.world1.y));
                let world2 = self
                    .camera
                    .to_global(Vector2::new(c.contact.world2.x, c.contact.world2.y));
                graphics::Line::new(color, 1.0).draw(
                    [world1.x, world1.y, world2.x, world2.y],
                    &graphics::DrawState::default(),
                    ctx.transform,
                    gfx,
                );
            }
        }

        match self.current_action {
            ActionKind::CreatingCircle if self.current_action_step == 1 => {
                let dist = nalgebra::distance(&self.mouse_position_world, &self.first_click_world);
                graphics::Ellipse::new(graphics::color::WHITE).draw(
                    [-dist, -dist, dist * 2.0, dist * 2.0],
                    &graphics::DrawState::default(),
                    ctx.trans(self.first_click.x, self.first_click.y)
                        .zoom(self.camera.zoom())
                        .transform,
                    gfx,
                );
            }
            ActionKind::CreatingRectangle if self.current_action_step == 1 => {
                let width = self.mouse_position_world.x - self.first_click_world.x;
                let height = self.mouse_position_world.y - self.first_click_world.y;
                graphics::Rectangle::new(graphics::color::WHITE).draw(
                    [-width, -height, width * 2.0, height * 2.0],
                    &graphics::DrawState::default(),
                    ctx.trans(self.first_click.x, self.first_click.y)
                        .zoom(self.camera.zoom())
                        .transform,
                    gfx,
                );
            }
            ActionKind::CreatingTriangle => {
                if self.current_action_step == 1 {
                    graphics::Line::new(graphics::color::BLACK, 1.0).draw(
                        [
                            self.first_click.x,
                            self.first_click.y,
                            self.mouse_position.x,
                            self.mouse_position.y,
                        ],
                        &graphics::DrawState::default(),
                        ctx.trans(self.first_click.x, self.first_click.y)
                            .zoom(self.camera.zoom())
                            .transform,
                        gfx,
                    );
                } else if self.current_action_step == 2 {
                }
            }
            _ => {}
        }
    }

    fn key(&mut self, key: Key, pressed: bool) {
        match key {
            Key::A | Key::Left if pressed => self.camera.trans(&Vector2::new(-10.0, 0.0)),
            Key::D | Key::Right if pressed => self.camera.trans(&Vector2::new(10.0, 0.0)),
            Key::W | Key::Up if pressed => self.camera.trans(&Vector2::new(0.0, -10.0)),
            Key::S | Key::Dollar if pressed => self.camera.trans(&Vector2::new(0.0, 10.0)),
            Key::Plus | Key::NumPadPlus if pressed => self.zoom_in(),
            Key::Minus | Key::NumPadMinus if pressed => self.zoom_out(),
            Key::Space if pressed => {
                self.running = !self.running;
                for part in &mut self.parts {
                    if self.running {
                        part.create(&mut self.world);
                    } else {
                        part.destroy(&mut self.world);
                    }
                }
            }
            _ => {}
        }
    }

    fn mouse(&mut self, button: MouseButton, pressed: bool) {
        self.middle_mouse_down = button == MouseButton::Middle && pressed;
        match button {
            MouseButton::Left if pressed => {
                if let Some(body) = util::get_body_at_mouse(&self.world, &self.mouse_position_world)
                {
                    self.grabbed_object = Some(body);
                    if let Some(joint) = self.grabbed_object_constraint {
                        let _ = self.world.remove_constraint(joint);
                    }

                    let body_part = self.world.body(body.0).unwrap().part(body.1).unwrap();
                    let body_pos = body_part.position();
                    let body_mass = body_part.local_inertia().mass();
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

                match self.current_action {
                    ActionKind::CreatingCircle => {
                        if self.current_action_step == 0 {
                            self.current_action_step += 1;
                            self.first_click = self.mouse_position;
                            self.first_click_world = self.mouse_position_world;
                            log::trace!(
                                "Setting first click to {},{}",
                                self.first_click.x,
                                self.first_click.y
                            );
                        } else if self.current_action_step == 1 {
                            let circle = part::ShapeBuilder::circle(nalgebra::distance(
                                &self.mouse_position_world,
                                &self.first_click_world,
                            ))
                            // temporary workaround
                            .position_p(self.first_click_world)
                            .build();
                            self.parts.push(part::Part::Shape(circle));
                            self.current_action = ActionKind::None;
                        }
                    }
                    ActionKind::CreatingRectangle => {
                        if self.current_action_step == 0 {
                            self.current_action_step += 1;
                            self.first_click = self.mouse_position;
                            self.first_click_world = self.mouse_position_world;
                            log::trace!(
                                "Setting first click to {},{}",
                                self.first_click.x,
                                self.first_click.y
                            );
                        } else if self.current_action_step == 1 {
                            let width = self.mouse_position_world.x - self.first_click_world.x;
                            let height = self.mouse_position_world.y - self.first_click_world.y;
                            let width = util::clamp(
                                width,
                                limits::MIN_RECTANGLE_SIZE,
                                limits::MAX_RECTANGLE_SIZE,
                            );
                            let height = util::clamp(
                                height,
                                limits::MIN_RECTANGLE_SIZE,
                                limits::MAX_RECTANGLE_SIZE,
                            );
                            let rectangle = part::ShapeBuilder::rectangle(width, height)
                                .position_p(self.first_click_world)
                                .build();
                            self.parts.push(part::Part::Shape(rectangle));
                            self.current_action = ActionKind::None;
                        }
                    }
                    ActionKind::CreatingTriangle => {
                        if self.current_action_step == 0 {
                            self.current_action_step += 1;
                            self.first_click = self.mouse_position;
                            self.first_click_world = self.mouse_position_world;
                            log::trace!(
                                "Setting first click to {},{}",
                                self.first_click.x,
                                self.first_click.y
                            );
                        } else if self.current_action_step == 1 {
                            self.current_action_step += 1;
                            self.second_click = self.mouse_position;
                            self.second_click_world = self.mouse_position_world;
                            log::trace!(
                                "Setting second click to {},{}",
                                self.second_click.x,
                                self.second_click.y
                            );
                        } else if self.current_action_step == 2 {
                            let triangle = part::ShapeBuilder::triangle(
                                nalgebra::Vector2::new(
                                    self.first_click_world.x,
                                    self.first_click_world.y,
                                ),
                                nalgebra::Vector2::new(
                                    self.second_click_world.x,
                                    self.second_click_world.y,
                                ),
                                nalgebra::Vector2::new(
                                    self.mouse_position_world.x,
                                    self.mouse_position_world.y,
                                ),
                            )
                            .build();
                            self.parts.push(part::Part::Shape(triangle));
                            self.current_action = ActionKind::None;
                        }
                    }
                    _ => {}
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

    fn mouse_cursor(&mut self, x: f64, y: f64) {
        self.mouse_position.x = x;
        self.mouse_position.y = y;
        let point = self.camera.to_local(self.mouse_position);
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

    fn mouse_relative(&mut self, x: f64, y: f64) {
        if self.middle_mouse_down && self.grabbed_object.is_none() {
            const MAX_MOVEMENT: f64 = 0.2;
            let x = if x > 0.0 {
                MAX_MOVEMENT
            } else if x < 0.0 {
                -MAX_MOVEMENT
            } else {
                x
            };
            let y = if y > 0.0 {
                MAX_MOVEMENT
            } else if y < 0.0 {
                -MAX_MOVEMENT
            } else {
                y
            };
            self.camera.trans(&Vector2::new(x, y));
        }
    }

    fn mouse_scroll(&mut self, x: f64, y: f64) {
        if y < 0.0 {
            self.zoom_out();
        } else {
            self.zoom_in();
        }
    }

    fn resize(&mut self, width: f64, height: f64) {
        self.camera.set_size(width, height);
    }
}
