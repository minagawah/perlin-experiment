use core::cell::RefCell;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::exit;
use crate::graphics::wave::WaveGraphics;
use crate::graphics::Graphics;
use crate::panels::wave::GraphType::{Bars, Radio, Solar};
use crate::panels::Panel;
use crate::types::Point;
use crate::utils::get_wrapper_element;

#[derive(Clone)]
pub struct WavePanel {
    id: String,
    g: Rc<RefCell<dyn Graphics>>,
    graph_type: Rc<Cell<GraphType>>,
}

impl Panel for WavePanel {
    fn g(&self) -> Rc<RefCell<dyn Graphics>> {
        self.g.clone()
    }

    fn draw(&mut self, points: &[Point], points_prev: &[Point], counter: u32) {
        if let Ok(mut g) = self.g.try_borrow_mut() {
            if let Some(g) = g.as_any_mut().downcast_mut::<WaveGraphics>() {
                g.clear();
                match self.graph_type.get() {
                    GraphType::Radio => g.render_radio(points, counter),
                    GraphType::Bars => g.render_bars(points, points_prev, counter),
                    GraphType::Solar => g.render_solar(points, points_prev, counter),
                }
            } else {
                exit("Failed to downcast_mut::<WaveGraphics>()");
            }
        } else {
            exit("Failed to borrow: self.g (WavePanel::draw)");
        }
    }
}

impl WavePanel {
    pub fn new(
        id: &str,
        width: f64,
        height: f64,
        bgcolor: &str,
        color: &str,
    ) -> Result<WavePanel, String> {
        let el: HtmlElement = get_wrapper_element(id)?;
        web_sys::console::log_1(&(format!("(wave) {} x {}", width as u32, height as u32).into()));

        let g: WaveGraphics = WaveGraphics::new(id, width, height, bgcolor, color)?;
        let graph_type: Rc<Cell<GraphType>> = Rc::new(Cell::new(GraphType::Radio));

        let graph_type_clone = graph_type.clone();
        let f = Closure::wrap(Box::new(move || {
            graph_type_clone.set(graph_type_clone.get().toggle());
        }) as Box<dyn FnMut()>);

        el.set_onclick(Some(f.as_ref().unchecked_ref()));
        f.forget();

        Ok(WavePanel {
            id: id.into(),
            g: Rc::new(RefCell::new(g)),
            graph_type,
        })
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
