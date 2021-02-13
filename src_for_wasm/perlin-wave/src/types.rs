use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Store {
    pub is_wave: bool,
}

impl Store {
    pub fn new() -> Self {
        Self { is_wave: false }
    }

    pub fn toggle(&mut self) {
        self.is_wave = !self.is_wave;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub color: String,
    pub color2: String,
}

#[derive(Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
