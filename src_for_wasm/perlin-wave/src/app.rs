use lerp::Lerp;
use rand::{self, Rng};

use crate::constants::{NORMAL_WIDTH, SEGMENTS};
use crate::panels::control::ControlPanel;
use crate::panels::wave::WavePanel;
use crate::panels::Panel;
use crate::perlin::noise_2d;
use crate::types::{Config, PanelConfig, Point};

pub struct App {
    points: Vec<Point>,
    points_prev: Vec<Point>,
    panels: Vec<Box<dyn Panel>>,
}

impl App {
    pub fn new(config: &Config) -> Result<App, String> {
        let bgcolor: String = config.bgcolor.clone();

        let mut panels = vec![];

        for panel in &config.panels {
            // Casting `PanelConfig` for `Vec<HashMap<String, String>>`
            // so that custom methods `ok` and `ok_f64` will be available.
            let panel: &dyn PanelConfig = panel;

            let id = panel.ok("id")?;
            let width = panel.ok_f64("width")?;
            let height = panel.ok_f64("height")?;
            let color = panel.ok("color")?;

            match id.as_str() {
                "wave" => {
                    let pane: Box<dyn Panel> = Box::new(WavePanel::new(
                        id.as_str(),
                        width,
                        height,
                        bgcolor.as_str(),
                        color.as_str(),
                    )?);
                    panels.push(pane);
                }
                "control" => {
                    let pane: Box<dyn Panel> = Box::new(ControlPanel::new(
                        id.as_str(),
                        width,
                        height,
                        bgcolor.as_str(),
                        color.as_str(),
                    )?);
                    panels.push(pane);
                }
                _ => {
                    return Err(format!("No such panel: {}", id));
                }
            }
        }

        Ok(App {
            points: vec![],
            points_prev: vec![],
            panels,
        })
    }

    pub fn reset(&mut self) {
        self.points_prev = if !self.points.is_empty() {
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
            self.points[i] = Point { x, y };
        }

        for panel in self.panels.iter_mut() {
            panel.reset();
        }
    }

    pub fn draw(&mut self, counter: u32) {
        for panel in self.panels.iter_mut() {
            panel.draw(&self.points, &self.points_prev, counter);
        }
    }
}
