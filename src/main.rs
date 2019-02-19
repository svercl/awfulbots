use glutin_window::{GlutinWindow, OpenGL};
use opengl_graphics::{GlGraphics, GlyphCache, TextureSettings};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{Button, ButtonArgs, ButtonState, Event, Input, Loop, Motion};
use piston::window::{AdvancedWindow, WindowSettings};

mod camera;
mod gui;
mod limits;
mod part;
mod state;
mod util;

use camera::Camera;
use gui::Gui;
use state::{GameState, State};

const INITIAL_WINDOW_WIDTH: f64 = 1280.0;
const INITIAL_WINDOW_HEIGHT: f64 = 720.0;

fn main() {
    // initialize logging facility
    env_logger::init();

    // this is a great middle ground
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow =
        WindowSettings::new("awfulbots", [INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT])
            .controllers(false)
            .opengl(opengl)
            .resizable(false)
            .build()
            .unwrap();

    let camera = Camera::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);
    let mut current_state: Box<State> = Box::new(GameState::new(camera));
    let mut gui = Gui::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);

    let mut gl = GlGraphics::new(opengl);
    let mut glyphs = GlyphCache::from_bytes(
        include_bytes!("../assets/ClearSans-Regular.ttf"),
        (),
        TextureSettings::new(),
    )
    .unwrap();

    let mut fps = fps_counter::FPSCounter::new();

    let mut events = Events::new(EventSettings::default().ups(60));
    while let Some(event) = events.next(&mut window) {
        gui.event(event.clone());

        match event {
            Event::Input(e) => match e {
                Input::Button(ButtonArgs { state, button, .. }) => match button {
                    Button::Keyboard(key) => current_state.key(key, state == ButtonState::Press),
                    Button::Mouse(mouse) => current_state.mouse(mouse, state == ButtonState::Press),
                    _ => {}
                },
                Input::Move(motion) => match motion {
                    Motion::MouseCursor(x, y) => current_state.mouse_cursor(x, y),
                    Motion::MouseRelative(x, y) => current_state.mouse_relative(x, y),
                    Motion::MouseScroll(x, y) => current_state.mouse_scroll(x, y),
                    _ => {}
                },
                Input::Resize(width, height) => current_state.resize(width, height),
                _ => {}
            },
            Event::Loop(e) => match e {
                Loop::Update(args) => {
                    current_state.update(args.dt);
                    gui.update();
                    window.set_title(format!(
                        "awfulbots | fps: {}, dt: {:.4}",
                        fps.tick(),
                        args.dt
                    ));
                }
                Loop::Render(args) => {
                    gl.draw(args.viewport(), |ctx, gfx| {
                        graphics::clear([0.2, 0.4, 0.6, 1.0], gfx);
                        current_state.draw(ctx, gfx, &mut glyphs);
                        gui.draw(ctx, gfx);
                    });
                }
                _ => {}
            },
            _ => {}
        }
    }
}
