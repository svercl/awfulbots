use glutin_window::GlutinWindow;
use glutin_window::OpenGL;
use opengl_graphics::GlGraphics;
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{Button, PressEvent, ReleaseEvent, RenderEvent, UpdateEvent};
use piston::window::{AdvancedWindow, WindowSettings};

mod camera;
mod limits;
mod part;
mod state;
mod util;

use camera::Camera;
use state::State;

fn main() {
    let mut state = State::new(Camera::new(800.0, 600.0));

    // this is a great middle ground
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("awfulbots", [800, 600])
        // don't close on esc (default)
        .exit_on_esc(false)
        // use specific opengl
        .opengl(opengl)
        // we don't want our user resizing the window (yet)
        .resizable(false)
        // vsync for smoothness (will eventually be a toggle)
        .vsync(true)
        .build()
        .unwrap();
    let mut gl = GlGraphics::new(opengl);

    let mut fps = fps_counter::FPSCounter::new();

    let mut events = Events::new(EventSettings::default().ups(60));
    while let Some(event) = events.next(&mut window) {
        if let Some(_) = event.update_args() {
            state.update();
            window.set_title(format!("awfulbots | fps: {}", fps.tick()));
        }

        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |c, g| {
                graphics::clear([0.2, 0.4, 0.6, 1.0], g);
                state.draw(c, g);
            });
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            state.key(key, true);
        }

        if let Some(Button::Keyboard(key)) = event.release_args() {
            state.key(key, false);
        }
    }
}
