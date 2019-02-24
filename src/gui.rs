use conrod_core::image::Map;
use conrod_core::text::rt::Rect;
use conrod_core::text::GlyphCache;
use conrod_core::widget::Widget;
use conrod_core::{Ui, UiBuilder, UiCell};
use opengl_graphics::{Format, GlGraphics, Texture, TextureSettings, UpdateTexture};
use piston::input::GenericEvent;

const SCALE_TOLERANCE: f32 = 0.1;
const POSITION_TOLERANCE: f32 = 0.1;

mod ids;

pub use self::ids::Ids;

// Gui is completely separate from the game
pub struct Gui {
    // the conrod Ui
    ui: Ui,
    // cached text vertex data
    text_vertex_data: Vec<u8>,
    // cached glyphs
    glyph_cache: GlyphCache<'static>,
    // cached text textures
    text_texture_cache: Texture,
    // ids for widgets
    ids: Ids,
    // map for images
    image_map: Map<Texture>,
    // the window width
    width: f64,
    // the window height
    height: f64,
}

impl Gui {
    pub fn new(width: f64, height: f64) -> Self {
        let mut ui = UiBuilder::new([width, height]).build();
        ui.fonts
            .insert_from_file("assets/ClearSans-Regular.ttf")
            .unwrap();

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

    pub fn ui_ids(&mut self) -> (UiCell, &Ids) {
        (self.ui.set_widgets(), &self.ids)
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

    pub fn draw(&mut self, ctx: graphics::Context, gfx: &mut GlGraphics) {
        // Avoid borrowchecker
        let Gui {
            ui,
            ref mut text_vertex_data,
            ref mut glyph_cache,
            ref mut text_texture_cache,
            ..
        } = self;

        let primitives = ui.draw();
        let cache_queued_glyphs =
            |_: &mut GlGraphics, cache: &mut Texture, rect: Rect<u32>, data: &[u8]| {
                let offset = [rect.min.x, rect.min.y];
                let size = [rect.width(), rect.height()];
                text_vertex_data.clear();
                text_vertex_data.extend(data.iter().flat_map(|&b| vec![255, 255, 255, b]));
                UpdateTexture::update(
                    cache,
                    &mut (), // we don't have a factory
                    Format::Rgba8,
                    &text_vertex_data[..],
                    offset,
                    size,
                )
                .unwrap();
            };

        fn texture_from_image<T>(img: &T) -> &T {
            img
        }

        conrod_piston::draw::primitives(
            primitives,
            ctx,
            gfx,
            text_texture_cache,
            glyph_cache,
            &self.image_map,
            cache_queued_glyphs,
            texture_from_image,
        );
    }
}
