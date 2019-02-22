use crate::gui::Ids;
use conrod_core::UiCell;
use graphics::Context;
use opengl_graphics::{GlGraphics, GlyphCache};
use piston::input::{Key, MouseButton};

mod game;
mod menu;

pub use self::game::GameScreen;
pub use self::menu::MenuScreen;

pub enum ScreenTransition {
    Continue,
    Change(Box<Screen>),
    Quit,
}

// the base of all other screens
pub trait Screen {
    // these MUST be implemented
    fn update(&mut self, dt: f64);
    fn update_gui(&mut self, _ui: &mut UiCell, _ids: &Ids) {}
    fn draw(&self, ctx: Context, gfx: &mut GlGraphics, glyphs: &mut GlyphCache<'static>);

    // these are optional
    fn key(&mut self, _key: Key, _pressed: bool) {}
    fn mouse(&mut self, _button: MouseButton, _pressed: bool) {}
    fn mouse_cursor(&mut self, _x: f64, _y: f64) {}
    fn mouse_relative(&mut self, _x: f64, _y: f64) {}
    fn mouse_scroll(&mut self, _x: f64, _y: f64) {}
    fn resize(&mut self, _width: f64, _height: f64) {}
}
