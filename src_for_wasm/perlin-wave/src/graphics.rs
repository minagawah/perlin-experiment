#![allow(unused_imports)]

use lerp::Lerp;
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::constants::{CONTROL_PANEL_RATIO, FILL_COLOR, FULL_CYCLE, SEGMENTS};
use crate::types::{Donut, Point};
use crate::utils::{create_canvas, ease_in_out_quad};

#[derive(Clone, Debug)]
pub struct Graphics {
    pub canvas: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,
    pub width: f64,
    pub height: f64,
    display_height: f64,
    control_height: f64,
    donut: Donut,
    color: String,
    color2: String,
    font_style: String,
    // flag: bool,
}

impl Graphics {
    pub fn new(
        id: &str,
        width: f64,
        height: f64,
        color: &str,
        color2: &str,
    ) -> Result<Self, String> {
        let canvas = create_canvas(id)?;

        canvas.set_width(width as u32);
        canvas.set_height(height as u32);

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let control_height: f64 = height * CONTROL_PANEL_RATIO;
        let display_height: f64 = height - control_height;
        let font_size: u32 = (control_height * 1.1) as u32;
        let font_style: String = format!("{}px serif", font_size);
        let donut = Donut::new(height, (SEGMENTS as f64 * 0.4).round());

        Ok(Self {
            canvas,
            ctx,
            width,
            height,
            display_height,
            control_height,
            donut,
            color: color.into(),
            color2: color2.into(),
            font_style,
            // flag: false,
        })
    }

    pub fn reset(self: &mut Graphics, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }

    pub fn clear(self: &mut Graphics) {
        self.ctx.clear_rect(0.0, 0.0, self.width, self.height);
        self.ctx.set_fill_style(&FILL_COLOR.into());
        self.ctx.fill_rect(0.0, 0.0, self.width, self.height);
    }

    pub fn render_wave(self: &mut Graphics, points: &Vec<Point>, counter: u32) {
        let half_h: f64 = self.display_height / 2.0;
        let amplify = self.amplifier();
        let rel_pos: f64 = ease_in_out_quad(self.relative_pos_half(counter));

        self.ctx.save();
        self.ctx.set_stroke_style(&self.color.as_str().into());
        self.ctx.begin_path();
        self.ctx.move_to(0_f64, half_h.round());

        for p in points {
            let x = p.x.round();
            let y = (0.0.lerp(p.y, rel_pos) * amplify + half_h).round();
            self.ctx.line_to(x, y);
        }

        self.ctx.line_to(self.width, half_h);
        self.ctx.stroke();
        self.ctx.restore();
    }

    pub fn render_bars(
        self: &mut Graphics,
        points: &Vec<Point>,
        points_prev: &Vec<Point>,
        counter: u32,
    ) {
        let unit_w: f64 = (self.width / SEGMENTS as f64) - 2.0;
        let half_h: f64 = self.display_height / 2.0;
        let amplify = self.amplifier();
        let rel_pos: f64 = self.relative_pos_full(counter);

        self.ctx.save();
        self.ctx.set_fill_style(&self.color.as_str().into());

        let mut i: usize = 0;
        for p in points {
            let x = p.x.round();
            let y = (points_prev[i].y.lerp(p.y, rel_pos) * amplify).round();
            let half_h = half_h.round();
            let unit_w = unit_w.round();
            self.ctx.fill_rect(x, half_h, unit_w, y);
            self.ctx.fill_rect(x, half_h, unit_w, -y);
            i += 1;
        }
        self.ctx.restore();
    }

    pub fn render_donut(
        self: &mut Graphics,
        points: &Vec<Point>,
        points_prev: &Vec<Point>,
        counter: u32,
    ) {
        let dn = self.donut.clone();
        let offset_x = self.width / 2.0;
        let offset_y = self.height / 2.0;
        let rel_pos: f64 = self.relative_pos_full(counter);

        self.ctx.save();
        self.ctx.set_fill_style(&self.color.as_str().into());
        self.ctx.translate(offset_x, offset_y).unwrap_or(());

        for i in 0..dn.segments as usize {
            let p = points[i].clone();
            let angle: f64 = i as f64 * dn.angle_step;
            let x = dn.radius_inner.round();
            let width: f64 = points_prev[i].normalize().y.lerp(p.normalize().y, rel_pos);
            let width = 0_f64.lerp(dn.max_length, width).round();
            let height = dn.size.round();
            let y = -(height / 2.0).round();

            self.ctx.save();
            self.ctx.rotate(angle * PI / 180.0).unwrap_or(());
            self.ctx.fill_rect(x, y, width, height);
            self.ctx.restore();
        }
        self.ctx.restore();
    }

    pub fn render_control(self: &mut Graphics, points: &Vec<Point>) {
        let text: String = format!("{:.5}", points[0].y.abs() * 10.0);
        self.ctx.save();
        self.ctx.set_fill_style(&self.color2.as_str().into());
        self.ctx.set_font(self.font_style.as_str());
        self.ctx
            .fill_text(text.as_str(), 5_f64, (self.display_height + 5.0).round())
            .unwrap_or(());
        self.ctx.restore();
    }

    fn amplifier(self: &mut Graphics) -> f64 {
        self.display_height * 0.2
    }

    fn relative_pos_full(self: &mut Graphics, counter: u32) -> f64 {
        counter as f64 / FULL_CYCLE
    }

    fn relative_pos_half(self: &mut Graphics, counter: u32) -> f64 {
        let half_cycle: f64 = FULL_CYCLE / 2.0;
        let pos: f64 = self.relative_pos_full(counter);
        if pos > 0.5 {
            1.0 - (counter as f64 - half_cycle) / half_cycle
        } else {
            counter as f64 / half_cycle
        }
    }
}
