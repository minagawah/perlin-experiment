use lerp::Lerp;
use rand::{self, Rng};

use crate::constants::{NORMAL_WIDTH, SEGMENTS};
use crate::panels::wave::WavePanel;
use crate::panels::Panel;
use crate::perlin::noise_2d;
use crate::types::{Config, Point};

pub struct App {
    points: Vec<Point>,
    points_prev: Vec<Point>,
    panels: Vec<Box<dyn Panel>>,
}

impl App {
    pub fn new(config: &Config) -> Result<App, String> {
        let color: String = config.color.clone();
        let color2: String = config.color2.clone();

        let mut panels = vec![];
        for panel in &config.panels {
            let id = panel.id.clone();
            match id.as_str() {
                "wave" => {
                    panels.push(Box::new(WavePanel::new(
                        id.as_str(),
                        color.as_str(),
                        color2.as_str(),
                    )?));
                }
                _ => {}
            }
        }

        Ok(App {
            points: vec![],
            points_prev: vec![],
            panels: panels,
        })
    }

    pub fn reset(&mut self) {
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

        for mut panel in self.panels.iter() {
            panel.reset();
        }
    }

    pub fn draw(&mut self, counter: u32) {
        for mut panel in self.panels.iter() {
            panel.draw(&self.points, &self.points_prev, counter);
        }
    }
}
