use std::sync::{Arc, Mutex};
use crate::base::{Rect, Size};
use crate::image::image_object::ImageObject;
use crate::ui::{ElementDelegate, ElementWeak};
use crate::ok_or_return;
use crate::render::RenderFn;
use crate::style::listener::LayoutListener;
use crate::style::measure::LayoutMeasurer;
use crate::style::node_item::MeasureParams;
use crate::style::StylePropKey;

#[derive(Clone)]
pub struct ImageDelegate {
    pub element: ElementWeak,
    pub img: Arc<Mutex<ImageObject>>,
}

impl ElementDelegate for ImageDelegate {
    fn handle_style_changed(&mut self, key: StylePropKey) {
        let element = ok_or_return!(self.element.upgrade());
        match key {
            StylePropKey::Color => {
                let changed = {
                    let mut img = self.img.lock().unwrap();
                    img.set_color(element.style.get_color())
                };
                if changed {
                    self.element.mark_dirty(false);
                }
            }
            _ => {}
        }
    }
    fn render(&mut self) -> RenderFn {
        self.img.lock().unwrap().render()
    }

}

impl LayoutListener for ImageDelegate {
    fn after_layout(&mut self, bounds: &Rect) {
        self.img.lock().unwrap().set_container_size((bounds.width, bounds.height));
    }
}

impl LayoutMeasurer for ImageDelegate {
    fn measure_layout(&mut self, params: MeasureParams) -> Size {
        let (width, height) = self.img.lock().unwrap().get_size();
        Size { width, height }
    }
}