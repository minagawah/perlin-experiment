#![allow(unused_imports)]

use lerp::Lerp;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::constants::{CONTROL_PANEL_RATIO, FILL_COLOR, FULL_CYCLE, SEGMENTS};
use crate::types::Point;
use crate::utils::{create_canvas, ease_in_out_quad};

#[derive(Clone, Debug)]
pub struct Graphics {
    pub canvas: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,
    pub width: f64,
    pub height: f64,
    control_height: f64,
    graph_height: f64,
    color: String,
    color2: String,
    font_style: String,
}

impl Graphics {
    pub fn new(id: &str, width: f64, height: f64, color: &str, color2: &str) -> Self {
        let canvas = create_canvas(id);
        canvas.set_width(width as u32);
        canvas.set_height(height as u32);

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let control_height: f64 = height * CONTROL_PANEL_RATIO;
        let graph_height: f64 = height - control_height;
        let font_size: u32 = (control_height * 1.1) as u32;
        let font_style: String = format!("{}px serif", font_size);

        Self {
            canvas,
            ctx,
            width,
            height,
            control_height,
            graph_height,
            color: color.into(),
            color2: color2.into(),
            font_style,
        }
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
        let half_h: f64 = self.graph_height / 2.0;
        self.ctx.set_stroke_style(&self.color.as_str().into());
        self.ctx.begin_path();
        self.ctx.move_to(0.0_f64, half_h);

        let amplify = self.graph_height * 0.5;
        let rel_y: f64 = ease_in_out_quad(self.relative_y_half(counter));
        for p in points {
            let y: f64 = 0.0.lerp(p.y, rel_y) * amplify + half_h;
            self.ctx.line_to(p.x, y);
        }
        self.ctx.line_to(self.width, half_h);
        self.ctx.stroke();
    }

    pub fn render_bars(
        self: &mut Graphics,
        points: &Vec<Point>,
        points_prev: &Vec<Point>,
        counter: u32,
    ) {
        let unit_w: f64 = (self.width / SEGMENTS as f64) - 2.0;
        let half_h: f64 = self.graph_height / 2.0;
        self.ctx.set_fill_style(&self.color.as_str().into());

        let amplify = self.graph_height * 0.2;
        let rel_y: f64 = self.relative_y_full(counter);
        let mut i: usize = 0;
        for p in points {
            let y: f64 = points_prev[i].y.lerp(p.y, rel_y) * amplify;
            self.ctx.fill_rect(p.x, half_h, unit_w, y);
            self.ctx.fill_rect(p.x, half_h, unit_w, -y);
            i += 1;
        }
    }

    pub fn render_control(self: &mut Graphics, points: &Vec<Point>) {
        let text: String = format!("{:.5}", points[0].y.abs() * 10.0);
        self.ctx.set_fill_style(&self.color2.as_str().into());
        self.ctx.set_font(self.font_style.as_str());
        self.ctx
            .fill_text(text.as_str(), 5.0_f64, self.graph_height + 5.0);
    }

    fn relative_y_full(self: &mut Graphics, counter: u32) -> f64 {
        counter as f64 / FULL_CYCLE
    }

    fn relative_y_half(self: &mut Graphics, counter: u32) -> f64 {
        let half_cycle: f64 = FULL_CYCLE / 2.0;
        let pos: f64 = self.relative_y_full(counter);
        if pos > 0.5 {
            1.0 - (counter as f64 - half_cycle) / half_cycle
        } else {
            counter as f64 / half_cycle
        }
    }
}
