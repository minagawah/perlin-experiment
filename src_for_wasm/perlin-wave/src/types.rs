use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Store {
    pub mode: usize,
}

impl Store {
    pub fn new() -> Self {
        Self { mode: 0 }
    }

    pub fn toggle(&mut self) {
        // let mode = (self.mode + 1) % 3;
        // web_sys::console::log_1(&(format!("mode: {}", mode).into()));
        // self.mode = mode;
        self.mode = (self.mode + 1) % 3;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub id: String,
    pub color: String,
    pub color2: String,
}

#[derive(Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn normalize(&self) -> Point {
        Point {
            x: (self.x + 1.0) / 2.0,
            y: (self.y + 1.0) / 2.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Donut {
    pub segments: f64,
    pub margin: f64,
    pub radius: f64,
    pub radius_inner: f64,
    pub max_length: f64,
    pub size: f64,
    pub angle_step: f64,
}

impl Donut {
    pub fn new(canvas_height: f64, segments: f64) -> Self {
        let angle_step = 360.0 / segments;
        let diameter = canvas_height * 0.9;
        let margin = (canvas_height - diameter) / 2.0;
        let radius = diameter / 2.0;
        let radius_inner = radius * 0.65;
        let max_length = radius - radius_inner;
        let size = diameter * PI / segments * 0.2;

        Donut {
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
