use core::cell::RefCell;
use std::rc::Rc;

use crate::exit;
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
        if let Ok(mut g) = self.g.try_borrow_mut() {
            if let Some(g) = g.as_any_mut().downcast_mut::<ControlGraphics>() {
                g.clear();
                g.render_control(&points);
            } else {
                exit("Faile to downcast_mut::<ControlGraphics>()");
            }
        } else {
            exit("Faile to borrow: self.g (ControlPanel::draw)");
        }
    }
}

impl ControlPanel {
    pub fn new(
        id: &str,
        width: f64,
        height: f64,
        bgcolor: &str,
        color: &str,
    ) -> Result<ControlPanel, String> {
        web_sys::console::log_1(
            &(format!("(control) {} x {}", width as u32, height as u32).into()),
        );

        let g = ControlGraphics::new(id, width, height, bgcolor, color)?;

        Ok(ControlPanel {
            id: id.into(),
            g: Rc::new(RefCell::new(g)),
        })
    }
}
