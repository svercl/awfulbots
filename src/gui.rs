use conrod_core::image::Map;
use conrod_core::text::rt::Rect;
use conrod_core::text::GlyphCache;
use conrod_core::{widget_ids, Ui, UiBuilder};
use opengl_graphics::{Format, GlGraphics, Texture, TextureSettings, UpdateTexture};
use piston::input::GenericEvent;

const SCALE_TOLERANCE: f32 = 0.1;
const POSITION_TOLERANCE: f32 = 0.1;

widget_ids! {
    struct Ids {
        button,
    }
}

pub struct Gui {
    ui: Ui,
    text_vertex_data: Vec<u8>,
    glyph_cache: GlyphCache<'static>,
    text_texture_cache: Texture,
    ids: Ids,
    image_map: Map<Texture>,
    width: f64,
    height: f64,
}

impl Gui {
    pub fn new(width: f64, height: f64) -> Self {
        let mut ui = UiBuilder::new([width, height]).build();

        let glyph_cache = GlyphCache::builder()
            .dimensions(width as u32, height as u32)
            .position_tolerance(POSITION_TOLERANCE)
            .scale_tolerance(SCALE_TOLERANCE)
            .build();

        let text_texture_cache = {
            let buf = vec![128; width as usize * height as usize];
            Texture::from_memory_alpha(&buf, width as u32, height as u32, &TextureSettings::new())
                .unwrap()
        };

        let ids = Ids::new(ui.widget_id_generator());

        Gui {
            ui,
            text_vertex_data: Vec::new(),
            glyph_cache,
            text_texture_cache,
            ids,
            image_map: Map::new(),
            width,
            height,
        }
    }

    pub fn event<GE>(&mut self, event: GE)
    where
        GE: GenericEvent,
    {
        if let Some([width, height]) = event.resize_args() {
            self.width = width;
            self.height = height;
        }

        if let Some(event) = conrod_piston::event::convert(event, self.width, self.height) {
            self.ui.handle_event(event);
        }
    }

    pub fn draw(&mut self, ctx: graphics::Context, gl: &mut GlGraphics) {
        if let Some(primitives) = self.ui.draw_if_changed() {
            let cache_queued_glyphs =
                |gl: &mut GlGraphics,
                 cache: &mut Texture,
                 rect: conrod_core::text::rt::Rect<u32>,
                 data: &[u8]| { unimplemented!() };
            fn texture_from_image<T>(img: &T) -> &T {
                img
            }
            conrod_piston::draw::primitives(
                primitives,
                ctx,
                gl,
                &mut self.text_texture_cache,
                &mut self.glyph_cache,
                &self.image_map,
                cache_queued_glyphs,
                texture_from_image,
            );
        }
    }
}
