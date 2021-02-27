use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::constants::CANVAS_RATIO;
use crate::graphics::waves::WavesGraphics;
use crate::graphics::Graphics;
use crate::panels::waves::GraphType::{Bars, Radio, Solar};
use crate::types::Point;
use crate::utils::get_wrapper_element;

#[derive(Clone, Debug)]
pub struct Waves {
    id: String,
    g: WavesGraphics,
    graph_type: Rc<Cell<GraphType>>,
}

impl Waves {
    pub fn new(id: &str, color: &str, color2: &str) -> Result<Self, String> {
        let el: HtmlElement = get_wrapper_element(id)?;
        let width: f64 = el.offset_width() as f64; // i32
        let height: f64 = width as f64 / CANVAS_RATIO;

        web_sys::console::log_1(&(format!(">> {} x {}", width, height).into()));

        let g: WavesGraphics = WavesGraphics::new(id, width, height, color, color2)?;
        let graph_type: Rc<Cell<GraphType>> = Rc::new(Cell::new(GraphType::Radio));

        let graph_type_clone = graph_type.clone();
        let f = Closure::wrap(Box::new(move || {
            graph_type_clone.set(graph_type_clone.get().toggle());
        }) as Box<dyn FnMut()>);

        el.set_onclick(Some(f.as_ref().unchecked_ref()));
        f.forget();

        Ok(Self {
            id: id.into(),
            g,
            graph_type,
        })
    }

    pub fn reset(&mut self) {
        self.g.reset(self.g.width, self.g.height);
    }

    pub fn draw(&mut self, points: &Vec<Point>, points_prev: &Vec<Point>, counter: u32) {
        self.g.clear();
        match self.graph_type.get() {
            GraphType::Radio => self.g.render_radio(&points, counter),
            GraphType::Bars => self.g.render_bars(&points, &points_prev, counter),
            GraphType::Solar => self.g.render_solar(&points, &points_prev, counter),
        }
        self.g.render_control(&points);
    }
}

#[derive(Copy, Clone, Debug)]
pub enum GraphType {
    Radio,
    Bars,
    Solar,
}

impl GraphType {
    pub fn toggle(&self) -> Self {
        web_sys::console::log_1(&(format!("toggle[0]: {:?}", self).into()));
        let res = match self {
            Radio => Bars,
            Bars => Solar,
            Solar => Radio,
        };
        web_sys::console::log_1(&(format!("toggle[1]: {:?}", res).into()));
        res
    }
}
