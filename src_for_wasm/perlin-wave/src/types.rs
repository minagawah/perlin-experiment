use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

/// Content of `panels` which is `HashMap<String, String>`
/// will later be explicitly cast to `PanelConfig`.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub bgcolor: String,
    pub panels: Vec<HashMap<String, String>>,
}

/// Provides custom methods for `HashMap<String, String>`.
pub trait PanelConfig {
    fn ok(&self, key: &str) -> Result<String, String>;
    fn ok_f64(&self, key: &str) -> Result<f64, String>;
}

impl PanelConfig for HashMap<String, String> {
    fn ok(&self, key: &str) -> Result<String, String> {
        match self.get(key) {
            Some(value) => Ok(value.to_string()),
            None => Err(format!("Failed to get: {}", key)),
        }
    }
    fn ok_f64(&self, key: &str) -> Result<f64, String> {
        let value = self.ok(key)?;
        f64::from_str(value.as_str()).map_err(|e| e.to_string())
    }
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
