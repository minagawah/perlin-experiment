pub mod app;
pub mod constants;
pub mod graphics;
pub mod panels;
pub mod perlin;
pub mod types;
pub mod utils;

// use log::error;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use crate::app::App;
use crate::constants::FULL_CYCLE;
use crate::types::Config;

pub fn exit(message: &str) {
    let v = wasm_bindgen::JsValue::from_str(&message.to_string());
    web_sys::console::log_1(&("panic".into()));
    web_sys::console::exception_1(&v);
    std::process::abort();
}

#[wasm_bindgen(start)]
pub fn start() {
    console_log::init().expect("console_log::init failed");
}

#[wasm_bindgen(js_name = "run")]
pub fn run(param: &JsValue) -> Result<(), JsValue> {
    // For all the unexpected `panic` are redirected to `console.error`.
    console_error_panic_hook::set_once();

    let config: Config = param.into_serde().unwrap();
    match start_app(&config) {
        Ok(_) => Ok(()),
        Err(err) => {
            // error!("Error: {}", err);
            web_sys::console::log_1(&("error".into()));
            // When `Err` is returned, `wasm_bindgen` will
            // automatically throw them as `Error` of JS.
            Err(JsValue::from(err))
        }
    }
}

pub fn start_app(config: &Config) -> Result<(), String> {
    let mut app = App::new(config)?;

    let mut counter: u32 = 0;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if counter > FULL_CYCLE as u32 {
            counter = 0;
        }
        if counter == 0 {
            app.reset();
        }
        app.draw(counter);

        counter += 1;

        utils::request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    utils::request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
