use crate::action::{Action, ActionKind};
use crate::camera::Camera;
use crate::limits;
use crate::part::{Joint, JointKind, Part, Shape, ShapeKind};
use crate::util;
use graphics::{Colored, Context, Transformed};
use nalgebra::{Isometry2, Point2, Vector2};
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

    fn draw_shape(
        &self,
        camera: &Camera,
        shape: &Shape,
        running: bool,
        ctx: Context,
        gfx: &mut GlGraphics,
    ) {
        let iso = if running {
            shape.world_iso()
        } else {
            shape.iso()
        };
        let color = shape.color();
        let (position, rotation) = (iso.translation.vector, iso.rotation.angle());
        let position = camera.to_global(position);
        let xf = ctx
            .trans(position.x, position.y)
            .rot_rad(rotation)
            .zoom(camera.zoom())
            .transform;
        match shape.kind() {
            ShapeKind::Circle { radius } => {
                use graphics::ellipse::Border;
                graphics::Ellipse::new(color)
                    .border(Border {
                        color: color.shade(0.5),
                        radius: 0.1,
                    })
                    .resolution(16)
                    .draw(
                        [-radius, -radius, radius * 2.0, radius * 2.0],
                        &graphics::DrawState::default(),
                        xf,
                        gfx,
                    );
            }
            ShapeKind::Rectangle {
                half_width,
                half_height,
            } => {
                use graphics::rectangle::Border;
                graphics::Rectangle::new(color)
                    .border(Border {
                        color: color.shade(0.5),
                        radius: 0.1,
                    })
                    .draw(
                        [
                            -half_width,
                            -half_height,
                            half_width * 2.0,
                            half_height * 2.0,
                        ],
                        &graphics::DrawState::default(),
                        xf,
                        gfx,
                    );
            }
            ShapeKind::Triangle { p1, p2, p3 } => {
                graphics::Polygon::new(color).draw(
                    &[[p1.x, p1.y], [p2.x, p2.y], [p3.x, p3.y]],
                    &graphics::DrawState::default(),
                    xf,
                    gfx,
                );
            }
        }
    }

    pub fn draw_parts(
        &self,
        camera: &Camera,
        parts: &[Part],
        running: bool,
        ctx: Context,
        gfx: &mut GlGraphics,
    ) {
        for part in parts {
            match part {
                Part::Shape(shape) => self.draw_shape(camera, shape, running, ctx, gfx),
                Part::Joint(_) => {}
            }
        }
    }
}
