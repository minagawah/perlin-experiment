use core::cell::RefCell;
use std::rc::Rc;

use crate::graphics::control::ControlGraphics;
use crate::graphics::Graphics;
use crate::panels::Panel;
use crate::types::Point;

#[derive(Clone)]
pub struct ControlPanel {
    id: String,
    g: Rc<RefCell<dyn Graphics>>,
}

impl Panel for ControlPanel {
    fn g(&self) -> Rc<RefCell<dyn Graphics>> {
        self.g.clone()
    }

    fn draw(&mut self, points: &[Point], _points_prev: &[Point], _counter: u32) {
        let mut g = self.g.borrow_mut();
        if let Some(g) = g.as_any_mut().downcast_mut::<ControlGraphics>() {
            g.clear();
            g.render_control(&points);
        }
    }
}

impl ControlPanel {
    pub fn new(id: &str, width: f64, height: f64, color: &str) -> Result<ControlPanel, String> {
        web_sys::console::log_1(&(format!(">> {} x {}", width as u32, height as u32).into()));

        let g: ControlGraphics = ControlGraphics::new(id, width, height, color)?;

        Ok(ControlPanel {
            id: id.into(),
            g: Rc::new(RefCell::new(g)),
        })
    }
}
