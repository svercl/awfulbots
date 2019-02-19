use crate::state::State;
use graphics::Context;
use opengl_graphics::{GlGraphics, GlyphCache};

pub struct MenuState {}

impl MenuState {
    pub fn new() -> Self {
        MenuState {}
    }
}

impl State for MenuState {
    fn update(&mut self, dt: f64) {}

    fn draw(&self, ctx: Context, gfx: &mut GlGraphics, glyphs: &mut GlyphCache) {}
}
