use crate as deft;
use deft_macros::mrc_object;
use crate::ui::{ElementDelegate, ElementWeak};
use crate::ok_or_return;
use crate::render::RenderFn;
use crate::style::StylePropKey;
use crate::text::textbox::TextBox;

#[mrc_object]
pub struct RichTextDelegate {
    pub element: ElementWeak,
    pub text_box: TextBox,
}

impl ElementDelegate for RichTextDelegate {
    fn handle_style_changed(&mut self, key: StylePropKey) {
        let ew = self.element.clone();
        let element = ok_or_return!(ew.upgrade());
        match key {
            StylePropKey::Color => {
                self.text_box.set_color(element.style.get_color());
            }
            StylePropKey::FontSize => {
                self.text_box.set_font_size(element.style.get_font_size());
            }
            StylePropKey::FontFamily => {
                self.text_box
                    .set_font_families(element.style.get_font_family().clone());
            }
            StylePropKey::FontWeight => {
                self.text_box.set_font_weight(element.style.get_font_weight());
            }
            StylePropKey::FontStyle => {
                self.text_box.set_font_style(element.style.get_font_style());
            }
            StylePropKey::LineHeight => {
                self.text_box.set_line_height(element.style.get_line_height());
            }
            _ => {}
        }
    }

    fn render(&mut self) -> RenderFn {
        self.text_box.render()
    }
}