mod delegate;

use std::sync::{Arc, Mutex};
use crate as deft;
use crate::js_module;
use crate::image::image_object::ImageObject;
use crate::ui::{Element, Widget};
use deft_macros::{widget, js_methods};
use yoga::Size;
use crate::ui::image::delegate::ImageDelegate;

#[widget]
pub struct Image {
    src: String,
    img: Arc<Mutex<ImageObject>>,
}

impl Widget for Image {}

js_module!(Image);

#[js_methods]
impl Image {

    #[js_func]
    pub fn set_src(&mut self, src: String) {
        self.update_img(ImageObject::new(&src));
    }

    pub fn set_src_svg_raw(&mut self, svg: &[u8]) {
        self.update_img(ImageObject::from_svg_bytes(svg));
    }

    fn update_img(&mut self, img: ImageObject) {
        *self.img.lock().unwrap() = img;
        self.el.mark_dirty(true);
    }

    #[js_func]
    pub fn create() -> Self {
        let element = Element::new("image");
        let mut img = Self {
            el: element,
            src: "".to_string(),
            img: Arc::new(Mutex::new(ImageObject::none())),
        };
        //TODO use weak ref?
        let img_obj = img.img.clone();
        img.el.style.set_measure_func(img_obj, |img, _params| {
                let (width, height) = img.lock().unwrap().get_size();
                return Size { width, height };
            });
        let element = img.el.as_weak();
        let img2 = img.img.clone();
        let delegate = ImageDelegate {
            element,
            img: img2,
        };
        img.el.set_layout_listener(delegate.clone());
        img.el.set_delegate(delegate);
        img
    }
}
