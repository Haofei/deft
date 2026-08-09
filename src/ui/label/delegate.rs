
use crate as deft;
use crate::base::Rect;
use crate::ui::{ElementDelegate, ElementWeak};
use crate::ok_or_return;
use crate::render::RenderFn;
use crate::style::StylePropKey;
use crate::text::textbox::TextBox;
use deft_macros::mrc_object;
use crate::style::listener::LayoutListener;

#[mrc_object]
pub struct LabelDelegate {
    pub element: ElementWeak,
    pub text_box: TextBox,
    pub layout_calculated: bool,
    pub text: String,
}

impl ElementDelegate for LabelDelegate {

    fn handle_style_changed(&mut self, key: StylePropKey) {
        let element = self.element.clone();
        let element = ok_or_return!(element.upgrade());
        match key {
            StylePropKey::Color => {
                let color = element.style.get_color();
                self.text_box.set_color(color);
                //TODO optimize dont relayout
                self.element.mark_dirty(true);
            }
            StylePropKey::FontSize => {
                let font_size = element.style.get_font_size();
                self.text_box.set_font_size(font_size);
                self.element.mark_dirty(true);
            }
            StylePropKey::FontFamily => {
                let font_families = element.style.get_font_family().clone();
                self.text_box.set_font_families(font_families);
                self.element.mark_dirty(true);
            }
            StylePropKey::FontWeight => {
                let font_weight = element.style.get_font_weight();
                self.text_box.set_font_weight(font_weight);
                self.element.mark_dirty(true);
            }
            StylePropKey::FontStyle => {
                let font_style = element.style.get_font_style();
                self.text_box.set_font_style(font_style);
                self.element.mark_dirty(true);
            }
            StylePropKey::LineHeight => {
                let line_height = element.style.get_line_height();
                self.text_box.set_line_height(line_height);
                self.element.mark_dirty(true);
            }
            _ => {}
        }
    }

    fn render(&mut self) -> RenderFn {
        let el = ok_or_return!(self.element.upgrade(), RenderFn::empty());
        let (pt, _, _, pl) = el.get_padding();
        let mut text_renderer = self.text_box.render();
        RenderFn::new(move |painter| {
            painter.canvas.translate((pl, pt));
            text_renderer.run(painter);
        })
    }

}

impl LayoutListener for LabelDelegate {
    fn before_layout(&mut self) {
        self.layout_calculated = false;
    }

    fn after_layout(&mut self, bounds: &Rect) {
        if !self.layout_calculated {
            self.text_box.set_layout_width(bounds.width);
            self.text_box.layout();
            self.layout_calculated = true;
        }
    }

}