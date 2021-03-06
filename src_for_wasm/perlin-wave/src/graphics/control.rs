use core::cell::RefCell;
use std::any::Any;
use std::rc::Rc;

use crate::exit;
use crate::graphics::Graphics;
use crate::types::Point;
use crate::utils::{get_canvas, get_ctx};

pub struct ControlGraphics {
    ctx: Rc<RefCell<web_sys::CanvasRenderingContext2d>>,
    pub width: f64,
    pub height: f64,
    bgcolor: String,
    color: String,
    font_size: u32,
    font_style: String,
}

impl Graphics for ControlGraphics {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn ctx(&mut self) -> Rc<RefCell<web_sys::CanvasRenderingContext2d>> {
        self.ctx.clone()
    }

    fn size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    fn bgcolor(&self) -> &str {
        &self.bgcolor
    }

    fn reset(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }
}

impl ControlGraphics {
    pub fn new(
        id: &str,
        width: f64,
        height: f64,
        bgcolor: &str,
        color: &str,
    ) -> Result<ControlGraphics, String> {
        let canvas = get_canvas(id, width, height)?;
        let ctx = get_ctx(&canvas)?;

        let font_size: u32 = (height * 0.8) as u32;
        let font_style: String = format!("{}px serif", font_size);

        Ok(ControlGraphics {
            ctx: Rc::new(RefCell::new(ctx)),
            width,
            height,
            bgcolor: bgcolor.into(),
            color: color.into(),
            font_size,
            font_style,
        })
    }

    pub fn render_control(&mut self, points: &[Point]) {
        let text: String = format!("{:.5}", points[0].y.abs() * 10.0);
        if let Ok(ctx) = self.ctx.try_borrow() {
            ctx.save();
            ctx.set_fill_style(&self.color.as_str().into());
            ctx.set_font(self.font_style.as_str());
            ctx.fill_text(text.as_str(), 5_f64, self.font_size as f64)
                .unwrap_or(());
            ctx.restore();
        } else {
            exit("Failed to borrow: self.ctx (render_control)");
        }
    }
}
