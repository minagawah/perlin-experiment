pub mod wave;

use core::cell::RefCell;
use std::rc::Rc;

use crate::graphics::Graphics;

pub trait Panel<G: Graphics> {
    fn g(&self) -> Rc<RefCell<G>>;

    fn reset(&mut self) {
        if let Ok(mut g) = self.g().try_borrow_mut() {
            let (width, height) = g.size();
            g.reset(width, height);
        };
    }
}
