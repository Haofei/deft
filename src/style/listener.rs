use crate::base::Rect;

pub trait LayoutListener {
    fn before_layout(&mut self) {}
    
    fn after_layout(&mut self, bounds: &Rect) {
        let _ = bounds;
    }
}

pub struct EmptyLayoutListener {
    
}

impl LayoutListener for EmptyLayoutListener {}