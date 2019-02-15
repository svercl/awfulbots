use conrod_core::image::Map;
use conrod_core::text::rt::Rect;
use conrod_core::text::GlyphCache;
use conrod_core::widget::{self, Widget};
use conrod_core::{widget_ids, Labelable, Positionable, Sizeable, Ui, UiBuilder};
use opengl_graphics::{Format, GlGraphics, Texture, TextureSettings, UpdateTexture};
use piston::input::GenericEvent;

const SCALE_TOLERANCE: f32 = 0.1;
const POSITION_TOLERANCE: f32 = 0.1;

widget_ids! {
    struct Ids {
        button,
        canvas,
        text
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
        let mut ui = UiBuilder::new([width, height]).theme(theme()).build();
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

    pub fn update(&mut self) {
        const MARGIN: f64 = 30.0;

        let mut ui = self.ui.set_widgets();
        widget::Canvas::new()
            .pad(MARGIN)
            .scroll_kids_vertically()
            .set(self.ids.canvas, &mut ui);
        widget::Text::new("Testing conrod")
            .mid_top_of(self.ids.canvas)
            .set(self.ids.text, &mut ui);
        widget::Button::new()
            .down(60.0)
            .label("PRESS ME")
            .set(self.ids.button, &mut ui);
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
                    &mut (), // we don't have an encoder
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
            gl,
            text_texture_cache,
            glyph_cache,
            &self.image_map,
            cache_queued_glyphs,
            texture_from_image,
        );
    }
}

fn theme() -> conrod_core::Theme {
    use conrod_core::position::{Align, Direction, Padding, Position, Relative};
    conrod_core::Theme {
        name: "Demo Theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod_core::color::DARK_CHARCOAL,
        shape_color: conrod_core::color::LIGHT_CHARCOAL,
        border_color: conrod_core::color::BLACK,
        border_width: 0.0,
        label_color: conrod_core::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: conrod_core::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}
