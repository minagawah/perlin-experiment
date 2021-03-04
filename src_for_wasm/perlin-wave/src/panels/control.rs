use core::cell::RefCell;
use std::rc::Rc;
use web_sys::HtmlElement;

use crate::constants::CONTROL_CANVAS_RATIO;
use crate::graphics::control::ControlGraphics;
use crate::graphics::Graphics;
use crate::panels::Panel;
use crate::types::Point;
use crate::utils::get_wrapper_element;

#[derive(Clone)]
pub struct ControlPanel {
    id: String,
    g: Rc<RefCell<dyn Graphics>>,
}

impl Panel for ControlPanel {
    fn g(&self) -> Rc<RefCell<dyn Graphics>> {
        self.g.clone()
    }

    fn draw(&mut self, points: &Vec<Point>, _points_prev: &Vec<Point>, _counter: u32) {
        let mut g = self.g.borrow_mut();
        match g.as_any_mut().downcast_mut::<ControlGraphics>() {
            Some(g) => {
                g.clear();
                g.render_control(&points);
            }
            None => {}
        }
    }
}

impl ControlPanel {
    pub fn new(id: &str, color: &str, color2: &str) -> Result<ControlPanel, String> {
        let el: HtmlElement = get_wrapper_element(id)?;
        let width: f64 = el.offset_width() as f64; // i32
        let height: f64 = width as f64 / CONTROL_CANVAS_RATIO;

        web_sys::console::log_1(&(format!(">> {} x {}", width, height).into()));

        let g: ControlGraphics = ControlGraphics::new(id, width, height, color, color2)?;

        Ok(ControlPanel {
            id: id.into(),
            g: Rc::new(RefCell::new(g)),
        })
    }
}
