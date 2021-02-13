use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// use rand_os::rand_core::RngCore;
// use rand_os::OsRng;

fn window() -> web_sys::Window {
    web_sys::window().unwrap()
}

fn document() -> web_sys::Document {
    window().document().unwrap()
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("Failed for requestAnimationFrame");
}

pub fn get_wrapper_element(name: &str) -> web_sys::HtmlElement {
    document()
        .get_element_by_id(name)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|_| ())
        .unwrap()
}

pub fn create_canvas(name: &str) -> web_sys::HtmlCanvasElement {
    let canvas = document()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    get_wrapper_element(name).append_child(&canvas).unwrap();
    canvas
}

pub fn ease_in_out_quad(v: f64) -> f64 {
    if v < 0.5 {
        v * v * 2.0
    } else {
        v * (4.0 - v * 2.0) - 1.0
    }
}

// pub fn osrng_gen() -> f64 {
//     let mut os_rng = OsRng::new().unwrap();
//     let mut seed = [0u8; 255];
//     os_rng.fill_bytes(&mut seed);
//     os_rng.next_u64() as f64
// }
