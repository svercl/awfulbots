use crate::action::{Action, ActionKind};
use crate::camera::Camera;
use crate::limits;
use crate::part::{Joint, JointKind, Part, Shape, ShapeKind};
use crate::util;
use graphics::{Context, Transformed};
use nalgebra::{Point2, Vector2};
use nphysics2d::world::World;
use opengl_graphics::GlGraphics;

pub struct Visualizer {}

impl Visualizer {
    pub fn new() -> Self {
        Visualizer {}
    }

    pub fn maybe_draw_action(
        &self,
        action: &Action,
        camera: &Camera,
        mouse_position: Vector2<f64>,
        mouse_position_world: Point2<f64>,
        ctx: Context,
        gfx: &mut GlGraphics,
    ) {
        match action.kind() {
            ActionKind::CreatingCircle if action.step() == 1 => {
                let radius = nalgebra::distance(&mouse_position_world, &action.first_click_world());
                let radius = util::clamp(radius, limits::MIN_CIRCLE_SIZE, limits::MAX_CIRCLE_SIZE);
                graphics::Ellipse::new(graphics::color::WHITE).draw(
                    [-radius, -radius, radius * 2.0, radius * 2.0],
                    &graphics::DrawState::default(),
                    ctx.trans(action.first_click().x, action.first_click().y)
                        .zoom(camera.zoom())
                        .transform,
                    gfx,
                );
            }
            ActionKind::CreatingRectangle if action.step() == 1 => {
                let width = mouse_position_world.x - action.first_click_world().x;
                let width = if width > 0.0 {
                    util::clamp(
                        width,
                        limits::MIN_RECTANGLE_SIZE,
                        limits::MAX_RECTANGLE_SIZE,
                    )
                } else {
                    util::clamp(
                        width,
                        -limits::MIN_RECTANGLE_SIZE,
                        -limits::MAX_RECTANGLE_SIZE,
                    )
                };
                let height = mouse_position_world.y - action.first_click_world().y;
                let height = if height > 0.0 {
                    util::clamp(
                        height,
                        limits::MIN_RECTANGLE_SIZE,
                        limits::MAX_RECTANGLE_SIZE,
                    )
                } else {
                    util::clamp(
                        height,
                        -limits::MIN_RECTANGLE_SIZE,
                        -limits::MAX_RECTANGLE_SIZE,
                    )
                };
                graphics::Rectangle::new(graphics::color::WHITE).draw(
                    [-width, -height, width * 2.0, height * 2.0],
                    &graphics::DrawState::default(),
                    ctx.trans(action.first_click().x, action.first_click().y)
                        .zoom(camera.zoom())
                        .transform,
                    gfx,
                );
            }
            ActionKind::CreatingTriangle => {
                if action.step() >= 1 {
                    graphics::Line::new(graphics::color::BLACK, 1.0).draw(
                        [
                            action.first_click().x,
                            action.first_click().y,
                            mouse_position.x,
                            mouse_position.y,
                        ],
                        &graphics::DrawState::default(),
                        ctx.transform,
                        gfx,
                    );
                    if action.step() == 2 {
                        graphics::Line::new(graphics::color::BLACK, 1.0).draw(
                            [
                                action.first_click().x,
                                action.first_click().y,
                                action.second_click().x,
                                action.second_click().y,
                            ],
                            &graphics::DrawState::default(),
                            ctx.transform,
                            gfx,
                        );
                        graphics::Line::new(graphics::color::BLACK, 1.0).draw(
                            [
                                action.second_click().x,
                                action.second_click().y,
                                mouse_position.x,
                                mouse_position.y,
                            ],
                            &graphics::DrawState::default(),
                            ctx.transform,
                            gfx,
                        );
                    }
                }
            }
            ActionKind::CreatingSlidingJoint => {
                if action.step() == 1 {
                    graphics::Line::new(graphics::color::BLACK, 1.0).draw(
                        [
                            action.first_click().x,
                            action.first_click().y,
                            mouse_position.x,
                            mouse_position.y,
                        ],
                        &graphics::DrawState::default(),
                        ctx.transform,
                        gfx,
                    );
                }
            }
            _ => {}
        }
    }

    pub fn draw_parts(&self, camera: &Camera, parts: &[Part], ctx: Context, gfx: &mut GlGraphics) {}

    pub fn draw_world(
        &self,
        camera: &Camera,
        world: &World<f64>,
        parts: &[Part],
        ctx: &Context,
        gfx: &mut GlGraphics,
    ) {
    }
}
