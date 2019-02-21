use crate::screen::Screen;
use graphics::Context;
use opengl_graphics::{GlGraphics, GlyphCache};

pub struct MenuScreen {}

impl MenuScreen {
    pub fn new() -> Self {
        MenuScreen {}
    }
}

impl Screen for MenuScreen {
    fn update(&mut self, dt: f64) {}
    fn draw(&self, ctx: Context, gfx: &mut GlGraphics, glyphs: &mut GlyphCache) {}
}
