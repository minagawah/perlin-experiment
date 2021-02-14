#![allow(unused_imports)]

use lerp::Lerp;
use rand::{self, Rng};
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use web_sys::HtmlElement;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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
    wrapper: HtmlElement,
    store: Rc<RefCell<Store>>,
}

impl App {
    pub fn new(config: &Config) -> Self {
        let id: String = config.id.clone();
        let color: String = config.color.clone();
        let color2: String = config.color2.clone();

        let wrapper: HtmlElement = get_wrapper_element(id.as_str());
        let wrapper_w: f64 = wrapper.offset_width() as f64; // i32
        let wrapper_h: f64 = wrapper_w as f64 / CANVAS_RATIO;

        web_sys::console::log_1(&(format!(">> {} x {}", wrapper_w, wrapper_h).into()));

        let g: Graphics = Graphics::new(
            id.as_str(),
            wrapper_w,
            wrapper_h,
            color.as_str(),
            color2.as_str()
        );

        let store: Rc<RefCell<Store>> = Rc::new(RefCell::new(Store::new()));

        Self {
            id,
            g,
            points: vec![],
            points_prev: vec![],
            wrapper,
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
        self.wrapper.set_onclick(Some(f.as_ref().unchecked_ref()));
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
