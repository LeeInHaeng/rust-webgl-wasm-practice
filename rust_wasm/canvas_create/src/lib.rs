use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, WebGlRenderingContext};
extern crate js_sys;

pub fn get_canvas(element_id: &str) -> HtmlCanvasElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(element_id).unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()
        .unwrap();

    canvas
}

#[wasm_bindgen]
pub fn create_canvas1() -> Result<(), JsValue> {
    let context = get_canvas("my_canvas3")
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    context.set_font("20pt Calibri");
    context.set_fill_style(&JsValue::from_str("green"));
    context.fill_text("Welcome to Tutorialspoint", 70.0, 70.0)
        .unwrap();

    Ok(())
}

#[wasm_bindgen]
pub fn create_canvas2() -> Result<(), JsValue> {
    let context = get_canvas("my_canvas4")
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap();

    context.clear_color(0.9, 0.9, 0.8, 1.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    Ok(())
}