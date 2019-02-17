use conrod_core::color;
use conrod_core::image::Map;
use conrod_core::text::rt::Rect;
use conrod_core::text::GlyphCache;
use conrod_core::widget::{self, Widget};
use conrod_core::{
    widget_ids, Color, Colorable, Labelable, Positionable, Scalar, Sizeable, Ui, UiBuilder,
};
use opengl_graphics::{Format, GlGraphics, Texture, TextureSettings, UpdateTexture};
use piston::input::GenericEvent;

const SCALE_TOLERANCE: f32 = 0.1;
const POSITION_TOLERANCE: f32 = 0.1;

widget_ids! {
    struct Ids {
        // top row
        circle_button,
        rectangle_button,
        triangle_button,
        undo_button,
        redo_button,
        zoom_in_button,
        save_button,
        main_menu_button,

        // bottom row
        fixed_joint_button,
        rotating_joint_button,
        sliding_joint_button,
        text_button,
        paste_button,
        zoom_out_button,
        load_button,

        canvas,
        text,
        slider,
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
        const MAIN_BUTTON_COLOR: Color = color::LIGHT_BLUE;
        const MAIN_BUTTON_WIDTH: Scalar = 80.0;
        const MAIN_BUTTON_HEIGHT: Scalar = 20.0;
        const UTILITY_BUTTON_COLOR: Color = color::LIGHT_ORANGE;
        const BUTTON_MARGIN: Scalar = 5.0;

        let mut ui = self.ui.set_widgets();
        widget::Canvas::new()
            .color(color::PURPLE)
            .h(65.0)
            .top_left()
            .set(self.ids.canvas, &mut ui);
        widget::Button::new()
            .color(MAIN_BUTTON_COLOR)
            .label_font_size(12)
            .label("Circle")
            .parent(self.ids.canvas)
            .top_left_with_margins(BUTTON_MARGIN, BUTTON_MARGIN)
            .wh([80.0, 20.0])
            .set(self.ids.circle_button, &mut ui);
        widget::Button::new()
            .color(MAIN_BUTTON_COLOR)
            .label_font_size(12)
            .label("Rectangle")
            .parent(self.ids.canvas)
            .right_from(self.ids.circle_button, BUTTON_MARGIN)
            .wh([80.0, 20.0])
            .set(self.ids.rectangle_button, &mut ui);
        widget::Button::new()
            .color(MAIN_BUTTON_COLOR)
            .label_font_size(12)
            .label("Triangle")
            .parent(self.ids.canvas)
            .right_from(self.ids.rectangle_button, BUTTON_MARGIN)
            .wh([80.0, 20.0])
            .set(self.ids.triangle_button, &mut ui);
        widget::Button::new()
            .color(UTILITY_BUTTON_COLOR)
            .label_font_size(12)
            .label("Undo")
            .parent(self.ids.canvas)
            .right_from(self.ids.triangle_button, BUTTON_MARGIN)
            .wh([60.0, 20.0])
            .set(self.ids.undo_button, &mut ui);
        widget::Button::new()
            .color(UTILITY_BUTTON_COLOR)
            .label_font_size(12)
            .label("Redo")
            .parent(self.ids.canvas)
            .right_from(self.ids.undo_button, BUTTON_MARGIN)
            .wh([60.0, 20.0])
            .set(self.ids.redo_button, &mut ui);

        widget::Button::new()
            .color(MAIN_BUTTON_COLOR)
            .down_from(self.ids.circle_button, BUTTON_MARGIN)
            .label_font_size(12)
            .label("Fixed Joint")
            .parent(self.ids.canvas)
            .wh([80.0, 20.0])
            .set(self.ids.fixed_joint_button, &mut ui);
        widget::Button::new()
            .color(MAIN_BUTTON_COLOR)
            .label_font_size(12)
            .label("Rotating Joint")
            .parent(self.ids.canvas)
            .right_from(self.ids.fixed_joint_button, BUTTON_MARGIN)
            .wh([80.0, 20.0])
            .set(self.ids.rotating_joint_button, &mut ui);
        widget::Button::new()
            .color(MAIN_BUTTON_COLOR)
            .label_font_size(12)
            .label("Sliding Joint")
            .parent(self.ids.canvas)
            .right_from(self.ids.rotating_joint_button, BUTTON_MARGIN)
            .wh([80.0, 20.0])
            .set(self.ids.sliding_joint_button, &mut ui);
        widget::Button::new()
            .color(MAIN_BUTTON_COLOR)
            .label_font_size(12)
            .label("Text")
            .parent(self.ids.canvas)
            .right_from(self.ids.sliding_joint_button, BUTTON_MARGIN)
            .wh([60.0, 20.0])
            .set(self.ids.text_button, &mut ui);
        if widget::Button::new()
            .color(UTILITY_BUTTON_COLOR)
            .label_font_size(12)
            .label("Paste")
            .parent(self.ids.canvas)
            .right_from(self.ids.text_button, BUTTON_MARGIN)
            .wh([60.0, 20.0])
            .set(self.ids.paste_button, &mut ui)
            .was_clicked()
        {
            log::trace!("Paste clicked");
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
            gl,
            text_texture_cache,
            glyph_cache,
            &self.image_map,
            cache_queued_glyphs,
            texture_from_image,
        );
    }
}
