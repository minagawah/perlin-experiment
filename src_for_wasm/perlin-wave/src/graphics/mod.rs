pub mod control;
pub mod wave;

use core::cell::RefCell;
use std::any::Any;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use crate::constants::FULL_CYCLE;
use crate::exit;

pub trait Graphics: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn ctx(&mut self) -> Rc<RefCell<web_sys::CanvasRenderingContext2d>>;

    fn size(&self) -> (f64, f64);

    fn bgcolor(&self) -> &str;

    fn clear(&mut self) {
        let (width, height) = self.size();
        if let Ok(ctx) = self.ctx().try_borrow() {
            ctx.clear_rect(0.0, 0.0, width, height);
            ctx.set_fill_style(&JsValue::from(self.bgcolor()));
            ctx.fill_rect(0.0, 0.0, width, height);
        } else {
            exit("Failed to borrow: self.ctx() (clear)");
        }
    }

    fn reset(&mut self, _width: f64, _height: f64) {}

    /// Having `FULL_CYCLE` representing the full-cycle,
    /// `counter` tells you where you are in the cycle.
    /// When `relative_pos_full` is fed with `counter`,
    /// it simply calculates for the ratio.
    /// `1.0` signifies it is at the end of the cycle,
    /// and `0.0` still at the beginning of the cycle.
    fn relative_pos_full(&self, counter: u32) -> f64 {
        counter as f64 / FULL_CYCLE
    }

    /// Similar to `relative_pos_full`, but `1.0` denotes
    /// it is in the middle of `FULL_CYCLE`.
    /// `0.5` means it is either approaching the middle
    /// or approaching the end of the cycle.
    /// `0.0` means it is either at the beginning,
    /// or at the end of the cycle.
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
