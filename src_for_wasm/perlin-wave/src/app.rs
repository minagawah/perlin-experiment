use lerp::Lerp;
use rand::{self, Rng};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::constants::{CANVAS_RATIO, SEGMENTS};
use crate::exit;
use crate::graphics::Graphics;
use crate::perlin::noise_2d;
use crate::types::{Config, Point, Store};
use crate::utils::get_wrapper_element;

#[derive(Clone, Debug)]
pub struct App {
    id: String,
    g: Graphics,
    points: Vec<Point>,
    points_prev: Vec<Point>,
    store: Rc<RefCell<Store>>,
}

impl App {
    pub fn new(config: &Config) -> Result<Self, String> {
        let id: String = config.id.clone();
        let color: String = config.color.clone();
        let color2: String = config.color2.clone();

        let el: HtmlElement = get_wrapper_element(id.as_str())?;

        let width: f64 = el.offset_width() as f64; // i32
        let height: f64 = width as f64 / CANVAS_RATIO;

        web_sys::console::log_1(&(format!(">> {} x {}", width, height).into()));

        let g: Graphics =
            Graphics::new(id.as_str(), width, height, color.as_str(), color2.as_str())?;

        let store: Rc<RefCell<Store>> = Rc::new(RefCell::new(Store::new()));
        let store_clone = store.clone();

        let f = Closure::wrap(Box::new(move || {
            if let Ok(mut s) = store_clone.try_borrow_mut() {
                s.toggle();
            } else {
                exit("Failed to borrow store.clone");
            }
        }) as Box<dyn FnMut()>);

        el.set_onclick(Some(f.as_ref().unchecked_ref()));
        f.forget();

        Ok(Self {
            id,
            g,
            points: vec![],
            points_prev: vec![],
            store,
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
            let x: f64 = 0_f64.lerp(self.g.width, ratio);
            let nx: f64 = x + offset;
            let y: f64 = noise_2d(nx, offset);
            self.points[i] = Point { x: x, y: y };
        }

        self.g.reset(self.g.width, self.g.height);
    }

    pub fn draw(self: &mut App, counter: u32) {
        self.g.clear();
        if let Ok(store) = self.store.try_borrow() {
            match store.mode {
                0 => self.g.render_wave(&self.points, counter),
                1 => self.g.render_bars(&self.points, &self.points_prev, counter),
                2 => self
                    .g
                    .render_solar(&self.points, &self.points_prev, counter),
                _ => (),
            }
        } else {
            exit("Failed to borrow self.store");
        }
        self.g.render_control(&self.points);
    }
}
