use core::cell::RefCell;
use lerp::Lerp;
use std::any::Any;
use std::f64::consts::PI;
use std::rc::Rc;

use crate::constants::{NORMAL_WIDTH, SEGMENTS};
use crate::types::Point;
use crate::utils::{ease_in_out_quad, get_canvas_ctx};

use crate::graphics::Graphics;

pub struct WaveGraphics {
    pub canvas: web_sys::HtmlCanvasElement,
    ctx: Rc<RefCell<web_sys::CanvasRenderingContext2d>>,
    pub width: f64,
    pub height: f64,
    solar_info: SolarInfo,
    color: String,
    color2: String,
}

impl Graphics for WaveGraphics {
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

    fn reset(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }
}

impl WaveGraphics {
    pub fn new(
        id: &str,
        width: f64,
        height: f64,
        color: &str,
        color2: &str,
    ) -> Result<WaveGraphics, String> {
        let (canvas, ctx) = get_canvas_ctx(id, width, height)?;

        let solar_info = SolarInfo::new(height, (SEGMENTS as f64 * 0.4).round());

        Ok(WaveGraphics {
            canvas,
            ctx: Rc::new(RefCell::new(ctx)),
            width,
            height,
            solar_info,
            color: color.into(),
            color2: color2.into(),
        })
    }

    fn amplify_value(&mut self) -> f64 {
        self.height * 0.2
    }

    pub fn render_radio(&mut self, points: &Vec<Point>, counter: u32) {
        let half_h: f64 = self.height / 2.0;
        let amplify = self.amplify_value();
        let rel_pos: f64 = ease_in_out_quad(self.relative_pos_half(counter));
        let ctx = self.ctx.borrow();

        ctx.save();
        ctx.set_stroke_style(&self.color.as_str().into());
        ctx.begin_path();
        ctx.move_to(0_f64, half_h.round());

        for p in points {
            let ratio = p.x / NORMAL_WIDTH;
            let x = 0_f64.lerp(self.width, ratio).round();
            let y = (0.0.lerp(p.y, rel_pos) * amplify + half_h).round();
            ctx.line_to(x, y);
        }

        ctx.line_to(self.width, half_h);
        ctx.stroke();
        ctx.restore();
    }

    pub fn render_bars(&mut self, points: &Vec<Point>, points_prev: &Vec<Point>, counter: u32) {
        let unit_w: f64 = (self.width / SEGMENTS as f64) - 2.0;
        let half_h: f64 = self.height / 2.0;
        let amplify = self.amplify_value();
        let rel_pos: f64 = self.relative_pos_full(counter);
        let ctx = self.ctx.borrow();

        ctx.save();
        ctx.set_fill_style(&self.color.as_str().into());

        let mut i: usize = 0;
        for p in points {
            let ratio = p.x / NORMAL_WIDTH;
            let x = 0_f64.lerp(self.width, ratio).round();
            let y = (points_prev[i].y.lerp(p.y, rel_pos) * amplify).round();
            let half_h = half_h.round();
            let unit_w = unit_w.round();
            ctx.fill_rect(x, half_h, unit_w, y);
            ctx.fill_rect(x, half_h, unit_w, -y);
            i += 1;
        }
        ctx.restore();
    }

    pub fn render_solar(&mut self, points: &Vec<Point>, points_prev: &Vec<Point>, counter: u32) {
        let sol = self.solar_info.clone();
        let offset_x = self.width / 2.0;
        let offset_y = self.height / 2.0;
        let rel_pos: f64 = self.relative_pos_full(counter);
        let ctx = self.ctx.borrow();

        ctx.save();
        ctx.set_fill_style(&self.color.as_str().into());
        ctx.translate(offset_x, offset_y).unwrap_or(());

        for i in 0..sol.segments as usize {
            let p = points[i].clone();
            let angle: f64 = i as f64 * sol.angle_step;
            let x = sol.radius_inner.round();
            let width: f64 = points_prev[i].normalize().y.lerp(p.normalize().y, rel_pos);
            let width = 0_f64.lerp(sol.max_length, width).round();
            let height = sol.size.round();
            let y = -(height / 2.0).round();

            ctx.save();
            ctx.rotate(angle * PI / 180.0).unwrap_or(());
            ctx.fill_rect(x, y, width, height);
            ctx.restore();
        }
        ctx.restore();
    }
}

#[derive(Clone)]
struct SolarInfo {
    segments: f64,
    margin: f64,
    radius: f64,
    radius_inner: f64,
    max_length: f64,
    size: f64,
    angle_step: f64,
}

impl SolarInfo {
    fn new(canvas_height: f64, segments: f64) -> SolarInfo {
        let angle_step = 360.0 / segments;
        let diameter = canvas_height * 0.9;
        let margin = (canvas_height - diameter) / 2.0;
        let radius = diameter / 2.0;
        let radius_inner = radius * 0.65;
        let max_length = radius - radius_inner;
        let size = diameter * PI / segments * 0.2;

        SolarInfo {
            segments,
            margin,
            radius,
            radius_inner,
            max_length,
            size,
            angle_step,
        }
    }
}
