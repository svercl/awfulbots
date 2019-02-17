use glutin_window::{GlutinWindow, OpenGL};
use opengl_graphics::{GlGraphics, GlyphCache, TextureSettings};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{Button, ButtonArgs, ButtonState, Event, Input, Loop, Motion};
use piston::window::{AdvancedWindow, WindowSettings};

mod camera;
mod game_state;
mod gui;
mod limits;
mod part;
mod util;

use camera::Camera;
use game_state::GameState;
use gui::Gui;

fn main() {
    // initialize logging facility
    env_logger::init();

    // our initial window size
    let initial_width = 1280.0;
    let initial_height = 720.0;

    let mut game_state = GameState::new(Camera::new(initial_width, initial_height));

    // this is a great middle ground
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow =
        WindowSettings::new("awfulbots", [initial_width, initial_height])
            .controllers(false)
            .opengl(opengl)
            .resizable(false)
            .build()
            .unwrap();
    let mut gl = GlGraphics::new(opengl);
    let mut gui = Gui::new(initial_width, initial_height);

    let mut glyphs =
        GlyphCache::new("assets/ClearSans-Regular.ttf", (), TextureSettings::new()).unwrap();

    let mut fps = fps_counter::FPSCounter::new();

    let mut events = Events::new(EventSettings::default().ups(60));
    while let Some(event) = events.next(&mut window) {
        gui.event(event.clone());

        match event {
            Event::Input(e) => match e {
                Input::Button(ButtonArgs { state, button, .. }) => match button {
                    Button::Keyboard(key) => game_state.key(key, state == ButtonState::Press),
                    Button::Mouse(mouse) => game_state.mouse(mouse, state == ButtonState::Press),
                    _ => {}
                },
                Input::Move(motion) => match motion {
                    Motion::MouseCursor(x, y) => game_state.mouse_cursor(x, y),
                    Motion::MouseRelative(x, y) => game_state.mouse_relative(x, y),
                    Motion::MouseScroll(x, y) => game_state.mouse_scroll(x, y),
                    _ => {}
                },
                Input::Resize(width, height) => game_state.resize(width, height),
                _ => {}
            },
            Event::Loop(e) => match e {
                Loop::Update(_) => {
                    game_state.update();
                    gui.update();
                    window.set_title(format!("awfulbots | fps: {}", fps.tick()));
                }
                Loop::Render(args) => {
                    gl.draw(args.viewport(), |ctx, gfx| {
                        graphics::clear([0.2, 0.4, 0.6, 1.0], gfx);
                        game_state.draw(ctx, gfx, &mut glyphs);
                        gui.draw(ctx, gfx);
                    });
                }
                _ => {}
            },
            _ => {}
        }

        // event.update(|_| {
        //     state.update();
        //     // gui.update();
        //     window.set_title(format!("awfulbots | fps: {}", fps.tick()));
        // });

        // event.render(|args| {
        //     gl.draw(args.viewport(), |c, g| {
        //         graphics::clear([0.2, 0.4, 0.6, 1.0], g);
        //         state.draw(c, g);
        //         // gui.draw(c, g);
        //     });
        // });

        // event.press(|button| match button {
        //     Button::Keyboard(key) => state.key(key, true),
        //     Button::Mouse(button) => state.mouse(button, true),
        //     _ => {}
        // });

        // event.release(|button| match button {
        //     Button::Keyboard(key) => state.key(key, false),
        //     Button::Mouse(button) => state.mouse(button, false),
        //     _ => {}
        // });

        // event.mouse_cursor(|x, y| state.mouse_cursor(x, y));
        // event.mouse_relative(|x, y| state.mouse_relative(x, y));
        // event.mouse_scroll(|x, y| state.mouse_scroll(x, y));
        // event.resize(|width, height| state.resize(width, height));
    }
}
