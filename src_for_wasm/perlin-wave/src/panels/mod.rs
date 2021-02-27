pub mod waves;

use core::cell::RefCell;
use std::rc::Rc;

use crate::graphics::waves::WavesGraphics;
use crate::graphics::Graphics;

pub trait Panel {
    fn g(&self) -> Rc<RefCell<WavesGraphics>>;

    fn reset(&mut self) {
        if let Ok(mut g) = self.g().try_borrow_mut() {
            let (width, height) = g.size();
            g.reset(width, height);
        };
    }
}
