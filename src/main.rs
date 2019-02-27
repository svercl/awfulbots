use glutin_window::{GlutinWindow, OpenGL};
use opengl_graphics::{GlGraphics, GlyphCache, TextureSettings};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{Button, ButtonArgs, ButtonState, Event, Input, Loop, Motion};
use piston::window::{AdvancedWindow, WindowSettings};

mod action;
mod camera;
mod gui;
mod limits;
mod part;
mod screen;
mod util;
mod visualizer;

use camera::Camera;
use gui::Gui;
use screen::{GameScreen, Screen};

const INITIAL_WINDOW_WIDTH: f64 = 1280.0;
const INITIAL_WINDOW_HEIGHT: f64 = 720.0;

fn main() {
    // initialize logging facility
    env_logger::init();

    // this is a great middle ground
    let opengl = OpenGL::V3_2;

    // create our window
    let mut window: GlutinWindow =
        WindowSettings::new("awfulbots", [INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT])
            // disable controllers
            .controllers(false)
            // use the same opengl for both glgraphics and the window
            .opengl(opengl)
            // don't allow resizing
            .resizable(false)
            .build()
            .expect("Unable to create window");

    let camera = Camera::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);
    // initialize with the game screen
    let mut current_screen: Box<Screen> = Box::new(GameScreen::new(camera));
    let mut gui = Gui::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);

    let mut gl = GlGraphics::new(opengl);
    let mut glyphs = GlyphCache::new("assets/ClearSans-Regular.ttf", (), TextureSettings::new())
        .expect("Unable to create glyph cache");

    // used to track fps
    let mut fps = fps_counter::FPSCounter::new();

    let mut events = Events::new(EventSettings::default().ups(60));
    while let Some(event) = events.next(&mut window) {
        // pass the event over to the gui for processing
        gui.event(event.clone());

        match event {
            Event::Input(e) => match e {
                Input::Button(ButtonArgs { state, button, .. }) => match button {
                    Button::Keyboard(key) => current_screen.key(key, state == ButtonState::Press),
                    Button::Mouse(mouse) => {
                        current_screen.mouse(mouse, state == ButtonState::Press)
                    }
                    _ => {}
                },
                Input::Move(motion) => match motion {
                    Motion::MouseCursor(x, y) => current_screen.mouse_cursor(x, y),
                    Motion::MouseRelative(x, y) => current_screen.mouse_relative(x, y),
                    Motion::MouseScroll(x, y) => current_screen.mouse_scroll(x, y),
                    _ => {}
                },
                Input::Resize(width, height) => current_screen.resize(width, height),
                _ => {}
            },
            Event::Loop(e) => match e {
                Loop::Update(args) => {
                    current_screen.update(args.dt);
                    let (mut ui, ids) = gui.ui_ids();
                    current_screen.update_gui(&mut ui, ids);
                    window.set_title(format!("awfulbots | fps: {}", fps.tick(),));
                }
                Loop::Render(args) => {
                    gl.draw(args.viewport(), |ctx, gfx| {
                        graphics::clear([0.2, 0.4, 0.6, 1.0], gfx);
                        current_screen.draw(ctx, gfx, &mut glyphs);
                        gui.draw(ctx, gfx);
                    });
                }
                _ => {}
            },
            _ => {}
        }
    }
}
