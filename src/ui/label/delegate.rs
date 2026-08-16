
use crate as deft;
use crate::base::{Rect, Size};
use crate::ui::{ElementDelegate, ElementWeak};
use crate::ok_or_return;
use crate::render::RenderFn;
use crate::style::StylePropKey;
use crate::text::textbox::{TextBox, TextElement, TextUnit};
use deft_macros::mrc_object;
use crate::event::TextUpdateEvent;
use crate::style::listener::LayoutListener;
use crate::style::measure::LayoutMeasurer;
use crate::style::node_item::MeasureParams;

#[mrc_object]
pub struct LabelDelegate {
    pub element: ElementWeak,
    pub text_box: TextBox,
    pub text: String,
    pub measure_called: bool,
    pub last_layout_width: Option<f32>,
}

impl LabelDelegate {

    pub fn new(ew: ElementWeak) -> Self {
        LabelDelegateData {
            text_box: TextBox::new(),
            measure_called: false,
            last_layout_width: None,
            element: ew,
            text: "".to_string(),
        }.to_ref()
    }

    pub fn set_text(&mut self, text: &str) {
        if self.text != text {
            self.text = text.to_string();
            self.text_box.clear();
            let text_unit = self.build_text_unit(text.to_string());
            self.text_box.add_line(vec![TextElement::Text(text_unit)]);
            self.make_layout_invalid();
            self.element.emit(TextUpdateEvent { value: text.to_string() })
        }
    }

    fn make_layout_invalid(&mut self) {
        self.last_layout_width = None;
        self.element.mark_dirty(true);
    }

    fn build_text_unit(&self, text: String) -> TextUnit {
        TextUnit {
            text,
            font_families: None,
            font_size: None,
            color: None,
            text_decoration_line: None,
            weight: None,
            background_color: None,
            style: None,
        }
    }

    fn do_layout(&mut self, width: f32) {
        // Skip relayout
        if self.last_layout_width == Some(width) {
            return;
        }
        self.text_box.set_layout_width(width);
        self.text_box.layout();
        self.last_layout_width = Some(width);
    }

}

impl ElementDelegate for LabelDelegate {

    fn handle_style_changed(&mut self, key: StylePropKey) {
        let element = self.element.clone();
        let element = ok_or_return!(element.upgrade());
        match key {
            StylePropKey::Color => {
                let color = element.style.computed().color();
                self.text_box.set_color(color);
                //TODO optimize dont relayout
                self.make_layout_invalid();
            }
            StylePropKey::FontSize => {
                let font_size = element.style.computed().font_size();
                self.text_box.set_font_size(font_size);
                self.make_layout_invalid();
            }
            StylePropKey::FontFamily => {
                let font_families = element.style.computed().font_family().clone();
                self.text_box.set_font_families(font_families);
                self.make_layout_invalid();
            }
            StylePropKey::FontWeight => {
                let font_weight = element.style.computed().font_weight();
                self.text_box.set_font_weight(font_weight);
                self.make_layout_invalid();
            }
            StylePropKey::FontStyle => {
                let font_style = element.style.computed().font_style().clone();
                self.text_box.set_font_style(font_style);
                self.make_layout_invalid();
            }
            StylePropKey::LineHeight => {
                let line_height = element.style.computed().line_height();
                self.text_box.set_line_height(line_height);
                self.make_layout_invalid();
            }
            _ => {}
        }
    }

    fn render(&mut self) -> RenderFn {
        let el = ok_or_return!(self.element.upgrade(), RenderFn::empty());
        let (pt, _, _, pl) = el.style.computed().padding();
        let mut text_renderer = self.text_box.render();
        RenderFn::new(move |painter| {
            painter.canvas.translate((pl, pt));
            text_renderer.run(painter);
        })
    }

}

impl LayoutListener for LabelDelegate {
    fn before_layout(&mut self) {
        self.measure_called = false;
    }

    fn after_layout(&mut self, bounds: &Rect) {
        if !self.measure_called {
            self.do_layout(bounds.width);
        }
    }

}

impl LayoutMeasurer for LabelDelegate {
    fn measure_layout(&mut self, params: MeasureParams) -> Size {
        self.measure_called = true;
        self.do_layout(params.width);
        let width = self.text_box.max_intrinsic_width();
        let height = self.text_box.height();
        // log::debug!("text measure params:{}x{}", params.width, params.height);
        // log::debug!("text measure result:{}x{}, {}", width, height, state.text_box.get_text());
        Size { width, height }
    }
}