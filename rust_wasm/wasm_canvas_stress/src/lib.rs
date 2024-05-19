use std::rc::Rc;
use std::cell::RefCell;
use rand::Rng;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
extern crate js_sys;

fn window() -> web_sys::Window {
    web_sys::window().unwrap()
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

fn get_canvas(element_id: &str) -> HtmlCanvasElement {
    let document = window().document().unwrap();
    let canvas = document.get_element_by_id(element_id).unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()
        .unwrap();

    canvas
}

struct Ball {
    x: f32,
    y: f32,
    x_speed: f32,
    y_speed: f32
}

impl Ball {
    fn new(canvas_width: f32, canvas_height: f32) -> Self {
        let mut rng = rand::thread_rng();
        let random_x: f32 = rng.gen();
        let random_y: f32 = rng.gen();
        let random_speed_x: f32 = rng.gen();
        let random_speed_y: f32 = rng.gen();

        let x_pos = random_x * (canvas_width - 40.0) + 20.0;
        let y_pos = random_y * (canvas_height - 40.0) + 300.0;
        let x_speed = random_speed_x * 4.0;
        let y_speed = random_speed_y * 4.0;

        Ball { x: x_pos, y: y_pos, x_speed: x_speed, y_speed: y_speed }
    }
}

fn draw_ball(ball: &Ball, ctx: &CanvasRenderingContext2d) {
    ctx.begin_path();
    ctx.arc(ball.x as f64, ball.y as f64, 20.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
    ctx.set_fill_style(&JsValue::from_str("green"));
    ctx.fill();
}

fn add_ball(canvas_width: f32, canvas_height: f32, balls: &mut Vec<Ball>, ball_count: &mut i32) {
    let ball = Ball::new(canvas_width, canvas_height);
    balls.push(ball);
    *ball_count += 1;
}

fn draw_ball_count(ctx: &CanvasRenderingContext2d, ball_count: i32, frame_rate: f64) {
    ctx.set_font("24px Arial");
    ctx.set_fill_style(&JsValue::from_str("green"));
    let fill_text_count = format!("공 개수: {}", ball_count);
    ctx.fill_text(&fill_text_count, 10.0, 30.0).unwrap();
    let fill_text_frame = format!("Frame rate: {} FPS", frame_rate);
    ctx.fill_text(&fill_text_frame, 10.0, 60.0).unwrap();
}

#[wasm_bindgen]
pub fn wasm_canvas_stress() -> Result<(), JsValue> {
    let canvas = get_canvas("my_Canvas");
    let ctx = canvas.get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let mut balls: Vec<Ball> = Vec::new();
    let mut ball_count = 0;
    let canvas_width = canvas.width();
    let canvas_height = canvas.height();

    let time_old = Rc::new(RefCell::new(0.0));
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f64| {
        ctx.clear_rect(0.0, 0.0, canvas_width as f64, canvas_height as f64);

        let delta_time = time - *time_old.borrow();
        let frame_rate = 1000.0 / delta_time;
        *time_old.borrow_mut() = time;

        for ball in balls.iter_mut() {
            ball.x += ball.x_speed;
            ball.y += ball.y_speed;
            if ball.x + 20.0 > canvas_width as f32 || ball.x - 20.0 < 0.0 {
                ball.x_speed = -ball.x_speed;
            }
            if ball.y + 20.0 > canvas_height as f32 || ball.y - 20.0 < 250.0 {
                ball.y_speed = -ball.y_speed;
            }
            draw_ball(ball, &ctx);
        }

        draw_ball_count(&ctx, ball_count, frame_rate);
        if ball_count < 5000 {
            add_ball(canvas_width as f32, canvas_height as f32, &mut balls, &mut ball_count);
        }

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(f64)>));
    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}