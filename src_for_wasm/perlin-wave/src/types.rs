use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PanelConfig {
    pub id: String,
    pub width: f64,
    pub height: f64,
    pub color: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub bgcolor: String,
    pub panels: Vec<PanelConfig>,
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
