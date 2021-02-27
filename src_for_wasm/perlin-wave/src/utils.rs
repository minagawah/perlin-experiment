use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// use rand_os::rand_core::RngCore;
// use rand_os::OsRng;

fn window() -> Result<web_sys::Window, String> {
    web_sys::window().ok_or("No window".into())
}

fn document() -> Result<web_sys::Document, String> {
    window()?.document().ok_or("No document".into())
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("Failed to start request_animation_frame");
}

pub fn get_wrapper_element(name: &str) -> Result<web_sys::HtmlElement, String> {
    let elem: web_sys::Element = document()?
        .get_element_by_id(name)
        .ok_or(format!("No element: {}", name).to_string())?;

    Ok(elem
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|e| e.to_string())?)
}

pub fn create_canvas(name: &str) -> Result<web_sys::HtmlCanvasElement, String> {
    let canvas = document()?
        .create_element("canvas")
        .map_err(|_| "Failed to create canvas".to_string())?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|e| e.to_string())?;

    get_wrapper_element(name)?
        .append_child(&canvas)
        .map_err(|_| "Failed append canvas".to_string())?;

    Ok(canvas)
}

pub fn get_canvas_ctx(
    id: &str,
    width: f64,
    height: f64,
) -> Result<
    (
        web_sys::HtmlCanvasElement,
        web_sys::CanvasRenderingContext2d,
    ),
    String,
> {
    let canvas = create_canvas(id)?;
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    Ok((canvas, ctx))
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
