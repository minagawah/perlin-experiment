use core::cell::RefCell;
use lerp::Lerp;
use std::any::Any;
use std::f64::consts::PI;
use std::rc::Rc;

use crate::constants::{NORMAL_WIDTH, SEGMENTS};
use crate::exit;
use crate::graphics::Graphics;
use crate::types::Point;
use crate::utils::{ease_in_out_quad, get_canvas, get_ctx};

pub struct WaveGraphics {
    ctx: Rc<RefCell<web_sys::CanvasRenderingContext2d>>,
    pub width: f64,
    pub height: f64,
    solar_info: SolarInfo,
    bgcolor: String,
    color: String,
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

    fn bgcolor(&self) -> &str {
        &self.bgcolor
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
        bgcolor: &str,
        color: &str,
    ) -> Result<WaveGraphics, String> {
        let canvas = get_canvas(id, width, height)?;
        let ctx = get_ctx(&canvas)?;
        let num_of_bars = (SEGMENTS as f64 * 0.4).round();
        let solar_info = SolarInfo::new(height, num_of_bars);

        Ok(WaveGraphics {
            ctx: Rc::new(RefCell::new(ctx)),
            width,
            height,
            solar_info,
            bgcolor: bgcolor.into(),
            color: color.into(),
        })
    }

    fn amplify_value(&mut self) -> f64 {
        self.height * 0.2
    }

    pub fn render_radio(&mut self, points: &[Point], counter: u32) {
        let half_h: f64 = self.height / 2.0;
        let amplify: f64 = self.amplify_value();
        let rel_pos: f64 = ease_in_out_quad(self.relative_pos_half(counter));

        if let Ok(ctx) = self.ctx.try_borrow() {
            ctx.save();
            ctx.set_stroke_style(&self.color.as_str().into());
            ctx.begin_path();
            ctx.move_to(0_f64, half_h.round());

            for p in points {
                let ratio = p.x / NORMAL_WIDTH;
                let x = 0_f64.lerp(self.width, ratio).round(); // MIN.lerp(MAX, ratio)
                let y = (0_f64.lerp(p.y, rel_pos) * amplify + half_h).round();
                ctx.line_to(x, y);
            }

            ctx.line_to(self.width, half_h);
            ctx.stroke();
            ctx.restore();
        } else {
            exit("Failed to borrow: self.ctx (render_radio)");
        }
    }

    pub fn render_bars(&mut self, points: &[Point], points_prev: &[Point], counter: u32) {
        let unit_w: f64 = (self.width / SEGMENTS as f64) - 2.0;
        let half_h: f64 = self.height / 2.0;
        let amplify: f64 = self.amplify_value();

        // Finding out where we are within `FULL_CYCLE`.
        // What we get is just a ratio between `0.0` and `1.0`.
        // `0.0` being at the beginning of the cycle,
        // and `1.0` being the end of the cycle.
        let rel_pos: f64 = self.relative_pos_full(counter);

        if let Ok(ctx) = self.ctx.try_borrow() {
            ctx.save();
            ctx.set_fill_style(&self.color.as_str().into());

            for (i, p) in points.iter().enumerate() {
                let ratio = p.x / NORMAL_WIDTH;
                let x = 0_f64.lerp(self.width, ratio).round();
                // We basically want to gradually move the bar
                // from the previous position to the current.
                // There, we need "rel_pos".
                let y = (points_prev[i].y.lerp(p.y, rel_pos) * amplify).round();
                let half_h = half_h.round();
                let unit_w = unit_w.round();
                // Drawing the upper part of the bar.
                ctx.fill_rect(x, half_h, unit_w, y);
                // Drawing the lower part of the bar.
                ctx.fill_rect(x, half_h, unit_w, -y);
            }
            ctx.restore();
        } else {
            exit("Failed to borrow: self.ctx (render_bars)");
        }
    }

    pub fn render_solar(&mut self, points: &[Point], points_prev: &[Point], counter: u32) {
        let sol = self.solar_info.clone();
        let offset_x = self.width / 2.0;
        let offset_y = self.height / 2.0;
        let rel_pos: f64 = self.relative_pos_full(counter);

        if let Ok(ctx) = self.ctx.try_borrow() {
            ctx.save();
            ctx.set_fill_style(&self.color.as_str().into());

            // First, moving the canvas to the center.
            ctx.translate(offset_x, offset_y).unwrap_or(());

            // The idea is to draw a horizontal bar
            // at 3 o'clock position. However,
            // everytime we draw the bar, we will
            // rotate canvas in different angle.
            // So, it is always a horizontal bar,
            // but because it is rotated,
            // we will have the effect of drawing
            // a flower like bars.
            for i in 0..sol.num_of_bars as usize {
                let p = points[i].clone();
                let angle: f64 = i as f64 * sol.angle_step;
                let x = sol.radius_inner.round();
                let width: f64 = points_prev[i].normalize().y.lerp(p.normalize().y, rel_pos);
                let width = 0_f64.lerp(sol.max_length, width).round();
                let height = sol.size.round();
                let y = -(height / 2.0).round();

                // Notice it `save` and `restore`
                // the canvas translation every time.
                ctx.save();
                ctx.rotate(angle * PI / 180.0).unwrap_or(());
                ctx.fill_rect(x, y, width, height);
                ctx.restore();
            }
            ctx.restore();
        } else {
            exit("Failed to borrow: self.ctx (render_solar)");
        }
    }
}

#[derive(Clone)]
struct SolarInfo {
    num_of_bars: f64,
    margin: f64,
    radius: f64,
    radius_inner: f64,
    max_length: f64,
    size: f64,
    angle_step: f64,
}

impl SolarInfo {
    fn new(canvas_height: f64, num_of_bars: f64) -> SolarInfo {
        let angle_step = 360.0 / num_of_bars;
        let diameter = canvas_height * 0.99;
        let margin = (canvas_height - diameter) / 2.0;
        let radius = diameter / 2.0;
        let radius_inner = radius * 0.55;
        let max_length = radius - radius_inner;
        let size = diameter * PI / num_of_bars * 0.2;

        web_sys::console::log_1(&(format!("(SolarInfo) num_of_bars: {}", num_of_bars).into()));
        web_sys::console::log_1(&(format!("(SolarInfo) size: {}", size).into()));

        SolarInfo {
            num_of_bars,
            margin,
            radius,
            radius_inner,
            max_length,
            size,
            angle_step,
        }
    }
}
