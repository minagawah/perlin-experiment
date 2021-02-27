use lerp::Lerp;
use rand::{self, Rng};

use crate::constants::{NORMAL_WIDTH, SEGMENTS};
use crate::panels::waves::WavesPanel;
use crate::panels::Panel;
use crate::perlin::noise_2d;
use crate::types::{Config, Point};

#[derive(Clone, Debug)]
pub struct App {
    points: Vec<Point>,
    points_prev: Vec<Point>,
    panels: Vec<WavesPanel>,
}

impl App {
    pub fn new(config: &Config) -> Result<Self, String> {
        let color: String = config.color.clone();
        let color2: String = config.color2.clone();
        let id: String = config.panels[0].id.clone();

        let mut panels = vec![];
        let waves = WavesPanel::new(id.as_str(), color.as_str(), color2.as_str())?;
        panels.push(waves.clone());

        Ok(App {
            points: vec![],
            points_prev: vec![],
            panels: panels,
        })
    }

    pub fn reset(self: &mut App) {
        self.points_prev = if self.points.len() > 0 {
            self.points.clone()
        } else {
            vec![Point { x: 0.0, y: 0.0 }; SEGMENTS]
        };

        self.points = vec![Point { x: 0.0, y: 0.0 }; SEGMENTS];

        let mut rng = rand::thread_rng();
        let offset = rng.gen_range(0, 10) as f64;
        for i in 0..SEGMENTS {
            let ratio = i as f64 / SEGMENTS as f64;
            let x: f64 = 0_f64.lerp(NORMAL_WIDTH, ratio);
            let nx: f64 = x + offset;
            let y: f64 = noise_2d(nx, offset);
            self.points[i] = Point { x: x, y: y };
        }

        for mut panel in self.panels.clone() {
            panel.reset();
        }
    }

    pub fn draw(self: &mut App, counter: u32) {
        for mut panel in self.panels.clone() {
            panel.draw(&self.points, &self.points_prev, counter);
        }
    }
}
