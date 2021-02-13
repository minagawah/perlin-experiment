#![allow(unused_imports)]

use lerp::Lerp;
use rand::{self, Rng};
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::constants::SEGMENTS;
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
    width: f64,
    height: f64,
    store: Rc<RefCell<Store>>,
}

impl App {
    pub fn new(config: &Config) -> Self {
        let id: String = config.id.clone();
        let width: u32 = config.width;
        let height: u32 = config.height;
        let color: &str = config.color.as_str();
        let color2: &str = config.color2.as_str();

        let g: Graphics = Graphics::new(id.as_str(), width, height, color, color2);
        let store: Rc<RefCell<Store>> = Rc::new(RefCell::new(Store { is_wave: false }));

        Self {
            id,
            g,
            points: vec![],
            points_prev: vec![],
            width: width as f64,
            height: height as f64,
            store,
        }
    }

    pub fn init(self: &mut App) {
        let store_clone = self.store.clone();
        let f = Closure::wrap(Box::new(move || {
            if let Ok(mut store) = store_clone.try_borrow_mut() {
                store.toggle();
            } else {
                exit("bad!");
            }
        }) as Box<dyn FnMut()>);

        let el = get_wrapper_element(self.id.as_str());
        el.set_onclick(Some(f.as_ref().unchecked_ref()));
        f.forget();
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
            let x: f64 = 0_f64.lerp(self.width, ratio);
            let nx: f64 = x + offset;
            let y: f64 = noise_2d(nx, offset);
            self.points[i] = Point { x: x, y: y };
        }

        self.g.reset(self.width, self.height);
    }

    pub fn draw(self: &mut App, counter: u32) {
        self.g.clear();
        if let Ok(store) = self.store.try_borrow() {
            if store.is_wave {
                self.g.render_wave(&self.points, counter);
            } else {
                self.g.render_bars(&self.points, &self.points_prev, counter);
            }
        } else {
            exit("bad!");
        }
        self.g.render_control(&self.points);
    }
}
