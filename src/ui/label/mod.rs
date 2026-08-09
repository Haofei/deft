mod delegate;

use crate as deft;
use crate::js_module;
use crate::ui::{Element, Widget};
use crate::event::TextUpdateEvent;
use crate::text::textbox::{TextBox, TextElement, TextUnit};
use deft_macros::{widget, js_methods};
use yoga::Size;
use crate::ui::label::delegate::{LabelDelegate, LabelDelegateData};

#[widget]
pub struct Label {
    delegate: LabelDelegate,
}

js_module!(Label);

#[js_methods]
impl Label {

    #[js_func]
    pub fn set_text(&mut self, text: String) {
        let old_text = self.get_text();
        if old_text != text {
            self.delegate.text = text.clone();
            self.delegate.text_box.clear();
            let text_unit = self.build_text_unit(text.clone());
            self.delegate
                .text_box
                .add_line(vec![TextElement::Text(text_unit)]);
            self.mark_dirty(true);

            self.el.emit(TextUpdateEvent { value: text })
        }
    }

    #[js_func]
    pub fn get_text(&self) -> String {
        self.delegate.text.clone()
    }

    fn mark_dirty(&mut self, layout_dirty: bool) {
        self.el.mark_dirty(layout_dirty);
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

    #[js_func]
    pub fn create() -> Self {
        let ele = Element::new("label");
        let state = LabelDelegateData {
            text_box: TextBox::new(),
            layout_calculated: false,
            element: ele.as_weak(),
            text: "".to_string(),
        }.to_ref();
        let mut label = Self {
            el: ele,
            delegate: state.clone(),
        };

        let state = label.delegate.clone();
        label.el.set_delegate(state.clone());
        let state = label.delegate.clone();
        label.el.set_layout_listener(state.clone());
        label.el.style.set_measure_func(state, |state, params| {
                state.text_box.set_layout_width(params.width);
                state.text_box.layout();
                state.layout_calculated = true;
                let width = state.text_box.max_intrinsic_width();
                let height = state.text_box.height();
                // log::debug!("text measure params:{}x{}", params.width, params.height);
                // log::debug!("text measure result:{}x{}, {}", width, height, state.text_box.get_text());
                return Size { width, height };
            });
        label
    }

}

impl Widget for Label {

}