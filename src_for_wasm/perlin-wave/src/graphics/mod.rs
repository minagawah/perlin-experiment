pub mod wave;

use core::cell::RefCell;
use std::rc::Rc;

use crate::constants::{FILL_COLOR, FULL_CYCLE};

pub trait Graphics {
    fn ctx(&mut self) -> Rc<RefCell<web_sys::CanvasRenderingContext2d>>;
    fn size(&self) -> (f64, f64);

    fn clear(&mut self) {
        let (width, height) = self.size();
        if let Ok(ctx) = self.ctx().try_borrow() {
            ctx.clear_rect(0.0, 0.0, width, height);
            ctx.set_fill_style(&FILL_COLOR.into());
            ctx.fill_rect(0.0, 0.0, width, height);
        }
    }

    fn relative_pos_full(&self, counter: u32) -> f64 {
        counter as f64 / FULL_CYCLE
    }

    fn relative_pos_half(&self, counter: u32) -> f64 {
        let half_cycle: f64 = FULL_CYCLE / 2.0;
        let pos: f64 = self.relative_pos_full(counter);
        if pos > 0.5 {
            1.0 - (counter as f64 - half_cycle) / half_cycle
        } else {
            counter as f64 / half_cycle
        }
    }
}
