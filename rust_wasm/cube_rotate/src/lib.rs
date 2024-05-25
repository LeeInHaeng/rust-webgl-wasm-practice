use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};
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

#[wasm_bindgen]
pub fn cube_rotate() -> Result<(), JsValue> {
    /*============= Creating a canvas =================*/
    let canvas = get_canvas("wasm_canvas");
    let gl = canvas.get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap();

    /*============ Defining and storing the geometry =========*/
    let vertices = [
        -1.0,-1.0,-1.0, 1.0,-1.0,-1.0, 1.0, 1.0,-1.0, -1.0, 1.0,-1.0,
        -1.0,-1.0, 1.0, 1.0,-1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        -1.0,-1.0,-1.0, -1.0, 1.0,-1.0, -1.0, 1.0, 1.0, -1.0,-1.0, 1.0,
        1.0,-1.0,-1.0, 1.0, 1.0,-1.0, 1.0, 1.0, 1.0, 1.0,-1.0, 1.0,
        -1.0,-1.0,-1.0, -1.0,-1.0, 1.0, 1.0,-1.0, 1.0, 1.0,-1.0,-1.0,
        -1.0, 1.0,-1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,-1.0, 
    ];

    let colors = [
        5.0,3.0,7.0, 5.0,3.0,7.0, 5.0,3.0,7.0, 5.0,3.0,7.0,
        1.0,1.0,3.0, 1.0,1.0,3.0, 1.0,1.0,3.0, 1.0,1.0,3.0,
        0.0,0.0,1.0, 0.0,0.0,1.0, 0.0,0.0,1.0, 0.0,0.0,1.0,
        1.0,0.0,0.0, 1.0,0.0,0.0, 1.0,0.0,0.0, 1.0,0.0,0.0,
        1.0,1.0,0.0, 1.0,1.0,0.0, 1.0,1.0,0.0, 1.0,1.0,0.0,
        0.0,1.0,0.0, 0.0,1.0,0.0, 0.0,1.0,0.0, 0.0,1.0,0.0
    ];

    let indices = [
        0,1,2, 0,2,3, 4,5,6, 4,6,7,
        8,9,10, 8,10,11, 12,13,14, 12,14,15,
        16,17,18, 16,18,19, 20,21,22, 20,22,23 
    ];

    // Create and store data into vertex buffer
    let vertex_buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    let vertices_array = unsafe {
        js_sys::Float32Array::view(&vertices)
    };
    gl.buffer_data_with_array_buffer_view(WebGlRenderingContext::ARRAY_BUFFER, &vertices_array, WebGlRenderingContext::STATIC_DRAW);

    // Create and store data into color buffer
    let color_buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&color_buffer));
    let colors_array = unsafe {
        js_sys::Float32Array::view(&colors)
    };
    gl.buffer_data_with_array_buffer_view(WebGlRenderingContext::ARRAY_BUFFER, &colors_array, WebGlRenderingContext::STATIC_DRAW);

    // Create and store data into index buffer
    let index_buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
    let indices_array = unsafe {
        js_sys::Uint16Array::view(&indices)
    };
    gl.buffer_data_with_array_buffer_view(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, &indices_array, WebGlRenderingContext::STATIC_DRAW);

    /*=================== Shaders =========================*/

    let vert_code = "
        attribute vec3 position;
        uniform mat4 Pmatrix;
        uniform mat4 Vmatrix;
        uniform mat4 Mmatrix;

        attribute vec3 color;
        varying vec3 vColor;
   
        void main(void) {
            gl_Position = Pmatrix*Vmatrix*Mmatrix*vec4(position, 1.);
           vColor = color;
        }
     ";

    let frag_code = "
        precision mediump float;
        varying vec3 vColor;
        void main(void) {
            gl_FragColor = vec4(vColor, 1.);
        }
    ";

    let vert_shader = gl.create_shader(WebGlRenderingContext::VERTEX_SHADER).unwrap();
    gl.shader_source(&vert_shader, &vert_code);
    gl.compile_shader(&vert_shader);

    let frag_shader = gl.create_shader(WebGlRenderingContext::FRAGMENT_SHADER).unwrap();
    gl.shader_source(&frag_shader, &frag_code);
    gl.compile_shader(&frag_shader);

    let shader_program = gl.create_program().unwrap();
    gl.attach_shader(&shader_program, &vert_shader);
    gl.attach_shader(&shader_program, &frag_shader);
    gl.link_program(&shader_program);

    /* ====== Associating attributes to vertex shader =====*/
    let p_matrix = gl.get_uniform_location(&shader_program, "Pmatrix").unwrap();
    let v_matrix = gl.get_uniform_location(&shader_program, "Vmatrix").unwrap();
    let m_matrix = gl.get_uniform_location(&shader_program, "Mmatrix").unwrap();

    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    let position = gl.get_attrib_location(&shader_program, "position");
    gl.vertex_attrib_pointer_with_f64(position as u32, 3, WebGlRenderingContext::FLOAT, false,0,0.0) ;

    // Position
    gl.enable_vertex_attrib_array(position as u32);
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&color_buffer));
    let color = gl.get_attrib_location(&shader_program, "color");
    gl.vertex_attrib_pointer_with_f64(color as u32, 3, WebGlRenderingContext::FLOAT, false,0,0.0) ;

    // Color
    gl.enable_vertex_attrib_array(color as u32);
    gl.use_program(Some(&shader_program));

    /*==================== MATRIX =====================*/
    let proj_matrix = get_projection(40.0, (canvas.width()/canvas.height()) as f32, 1.0, 100.0);

    let mut mov_matrix = [1.0,0.0,0.0,0.0, 0.0,1.0,0.0,0.0, 0.0,0.0,1.0,0.0, 0.0,0.0,0.0,1.0];
    let mut view_matrix = [1.0,0.0,0.0,0.0, 0.0,1.0,0.0,0.0, 0.0,0.0,1.0,0.0, 0.0,0.0,0.0,1.0];

    // translating z
    view_matrix[14] = view_matrix[14]-6.0;//zoom

    /*================= Drawing ===========================*/
    let time_old = Rc::new(RefCell::new(0.0));
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f64| {
        /* requestAnimationFrame end 조건이 있을 경우
        if requestAnimationFrame end {
            let _ = f.borrow_mut().take();
            return;
        }
        */
        let dt = time - *time_old.borrow();
        rotate_z(&mut mov_matrix, dt as f32 * 0.005);
        rotate_y(&mut mov_matrix, dt as f32*0.002);
        rotate_x(&mut mov_matrix, dt as f32*0.003);
        *time_old.borrow_mut() = time;

        gl.enable(WebGlRenderingContext::DEPTH_TEST);
        gl.depth_func(WebGlRenderingContext::LEQUAL);
        gl.clear_color(0.5, 0.5, 0.5, 0.9);
        gl.clear_depth(1.0);
        gl.viewport(0, 0, canvas.width().try_into().unwrap(), canvas.height().try_into().unwrap());
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);

        gl.uniform_matrix4fv_with_f32_array(Some(&p_matrix), false, &proj_matrix);
        gl.uniform_matrix4fv_with_f32_array(Some(&v_matrix), false, &view_matrix);
        gl.uniform_matrix4fv_with_f32_array(Some(&m_matrix), false, &mov_matrix);

        gl.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        gl.draw_elements_with_i32(WebGlRenderingContext::TRIANGLES, indices.len() as i32, WebGlRenderingContext::UNSIGNED_SHORT, 0);

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(f64)>));
    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn get_projection(angle: f32, a: f32, z_min: f32, z_max: f32) -> [f32; 16] {
    let ang = (angle * 0.5).to_radians().tan();

    [
        0.5/ang, 0.0, 0.0, 0.0,
        0.0, 0.5*a/ang, 0.0, 0.0,
        0.0, 0.0, -(z_max + z_min)/(z_max - z_min), -1.0,
        0.0, 0.0, (-2.0*z_max*z_min) as f32/(z_max-z_min), 0.0
    ]
}

/*==================== Rotation ====================*/
fn rotate_z(matrix: &mut [f32; 16], angle: f32) {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();
    let mv0 = matrix[0];
    let mv4 = matrix[4];
    let mv8 = matrix[8];

    matrix[0] = cos_angle*matrix[0] - sin_angle*matrix[1];
    matrix[4] = cos_angle*matrix[4] - sin_angle*matrix[5];
    matrix[8] = cos_angle*matrix[8] - sin_angle*matrix[9];
    matrix[1] = cos_angle*matrix[1] + sin_angle*mv0;
    matrix[5] = cos_angle*matrix[5] + sin_angle*mv4;
    matrix[9] = cos_angle*matrix[9] + sin_angle*mv8;
}

fn rotate_x(matrix: &mut [f32; 16], angle: f32) {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();
    let mv1 = matrix[1];
    let mv5 = matrix[5];
    let mv9 = matrix[9];

    matrix[1] = matrix[1]*cos_angle-matrix[2]*sin_angle;
    matrix[5] = matrix[5]*cos_angle-matrix[6]*sin_angle;
    matrix[9] = matrix[9]*cos_angle-matrix[10]*sin_angle;

    matrix[2] = matrix[2]*cos_angle+mv1*sin_angle;
    matrix[6] = matrix[6]*cos_angle+mv5*sin_angle;
    matrix[10] = matrix[10]*cos_angle+mv9*sin_angle;
 }

 fn rotate_y(matrix: &mut [f32; 16], angle: f32) {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();
    let mv0 = matrix[0];
    let mv4 = matrix[4];
    let mv8 = matrix[8];

    matrix[0] = cos_angle*matrix[0]+sin_angle*matrix[2];
    matrix[4] = cos_angle*matrix[4]+sin_angle*matrix[6];
    matrix[8] = cos_angle*matrix[8]+sin_angle*matrix[10];

    matrix[2] = cos_angle*matrix[2]-sin_angle*mv0;
    matrix[6] = cos_angle*matrix[6]-sin_angle*mv4;
    matrix[10] = cos_angle*matrix[10]-sin_angle*mv8;
 }