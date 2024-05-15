use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};
extern crate js_sys;

pub fn get_canvas(element_id: &str) -> HtmlCanvasElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(element_id).unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()
        .unwrap();

    canvas
}

#[wasm_bindgen]
pub fn draw_triangle_trans() -> Result<WebGlRenderingContext, JsValue> {

    /*=================Creating a canvas=========================*/
    let canvas = get_canvas("wasm_canvas");
    let gl = canvas.get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap();

    /*===========Defining and storing the geometry==============*/
    let vertices = [
        -0.5, 0.5, 0.0, 	
        -0.5, -0.5, 0.0, 	
        0.5, -0.5, 0.0
    ];

    //Create an empty buffer object and store vertex data            
    let vertex_buffer = gl.create_buffer()
        .unwrap();

    //Create a new buffer
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    
    //bind it to the current buffer
    let vertices_array = unsafe {
        js_sys::Float32Array::view(&vertices)
    };
    gl.buffer_data_with_array_buffer_view(WebGlRenderingContext::ARRAY_BUFFER, &vertices_array, WebGlRenderingContext::STATIC_DRAW); 
    
    // Pass the buffer data
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);

    /*========================Shaders============================*/
    
    //vertex shader source code 
    let vert_code = "
        attribute vec4 coordinates;
        uniform vec4 translation;
        void main(void) {
            gl_Position = coordinates + translation;
        }
    ";

    //Create a vertex shader program object and compile it              
    let vert_shader = gl.create_shader(WebGlRenderingContext::VERTEX_SHADER)
        .unwrap();
    gl.shader_source(&vert_shader, vert_code);
    gl.compile_shader(&vert_shader);

    //fragment shader source code
    let frag_code = "
        void main(void) {
            gl_FragColor = vec4(0.0, 0.0, 0.0, 0.1);
        }
    ";

    //Create a fragment shader program object and compile it            
    let frag_shader = gl.create_shader(WebGlRenderingContext::FRAGMENT_SHADER)
        .unwrap();
    gl.shader_source(&frag_shader, frag_code);
    gl.compile_shader(&frag_shader);

    //Create and use combiened shader program
    let shader_program = gl.create_program()
        .unwrap();
    gl.attach_shader(&shader_program, &vert_shader);
    gl.attach_shader(&shader_program, &frag_shader);
    gl.link_program(&shader_program);

    gl.use_program(Some(&shader_program));

    /* ===========Associating shaders to buffer objects============*/

    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    let coord = gl.get_attrib_location(&shader_program, "coordinates");
    gl.vertex_attrib_pointer_with_f64(coord as u32, 3, WebGlRenderingContext::FLOAT, false, 0, 0.0);
    gl.enable_vertex_attrib_array(coord as u32);

    /* ==========translation======================================*/
    let tx = 0.5;
    let ty = 0.5;
    let tz = 0.0;

    let translation = gl.get_uniform_location(&shader_program, "translation")
        .unwrap();
    gl.uniform4f(Some(&translation), tx, ty, tz, 0.0);

    /*=================Drawing the riangle and transforming it========================*/ 

    gl.clear_color(0.5, 0.5, 0.5, 0.9);
    gl.enable(WebGlRenderingContext::DEPTH_TEST);

    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    gl.viewport(0, 0, canvas.width().try_into().unwrap(), canvas.height().try_into().unwrap());
    gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 3);

    Ok(gl)
}