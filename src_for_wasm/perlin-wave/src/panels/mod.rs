pub mod control;
pub mod wave;

use core::cell::RefCell;
use std::rc::Rc;

use crate::graphics::Graphics;
use crate::types::Point;

pub trait Panel {
    fn g(&self) -> Rc<RefCell<dyn Graphics>>;

    fn reset(&mut self) {
        if let Ok(mut g) = self.g().try_borrow_mut() {
            let (width, height) = g.size();
            g.reset(width, height);
        };
    }

    fn draw(&mut self, points: &[Point], points_prev: &[Point], counter: u32);
}
