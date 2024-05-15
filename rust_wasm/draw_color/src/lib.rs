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
pub fn draw_color() -> Result<WebGlRenderingContext, JsValue> {
    /*============= Creating a canvas ==================*/
    let canvas = get_canvas("wasm_canvas");
    let gl = canvas.get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap();

    let vertices = [
        -0.5, 0.5, 0.0,
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.5, 0.5, 0.0
    ];

    let colors = [
        0.0, 0.0, 1.0, 
        1.0, 0.0, 0.0, 
        0.0, 1.0, 0.0, 
        1.0, 0.0, 1.0
    ];

    let indices = [3, 2, 1, 3, 1, 0];

    // Create an empty buffer object and store vertex data
    let vertex_buffer = gl.create_buffer()
        .unwrap();
    let vertices_array = unsafe {
        js_sys::Float32Array::view(&vertices)
    };
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    gl.buffer_data_with_array_buffer_view(WebGlRenderingContext::ARRAY_BUFFER, &vertices_array, WebGlRenderingContext::STATIC_DRAW);
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);

    // Create an empty buffer object and store Index data
    let index_buffer = gl.create_buffer()
        .unwrap();
    let indicies_array = unsafe {
        js_sys::Uint16Array::view(&indices)
    };
    gl.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
    gl.buffer_data_with_array_buffer_view(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, &indicies_array, WebGlRenderingContext::STATIC_DRAW);
    gl.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, None);

    // Create an empty buffer object and store color data
    let color_buffer = gl.create_buffer()
        .unwrap();
    let colors_array = unsafe {
        js_sys::Float32Array::view(&colors)
    };
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&color_buffer));
    gl.buffer_data_with_array_buffer_view(WebGlRenderingContext::ARRAY_BUFFER, &colors_array, WebGlRenderingContext::STATIC_DRAW);

    /*======================= Shaders =======================*/

    // vertex shader source code
    let vert_code = "
        attribute vec3 coordinates;
        attribute vec3 color;
        varying vec3 vColor;
        void main(void) {
            gl_Position = vec4(coordinates, 1.0);
            vColor = color;
        }
    ";

    // Create a vertex shader object
    let vert_shader = gl.create_shader(WebGlRenderingContext::VERTEX_SHADER)
        .unwrap();

    // Attach vertex shader source code
    gl.shader_source(&vert_shader, vert_code);

    // Compile the vertex shader
    gl.compile_shader(&vert_shader);

    // fragment shader source code
    let frag_code = "
        precision mediump float;
        varying vec3 vColor;
        void main(void) {
            gl_FragColor = vec4(vColor, 1.);
        }
    ";
    
    // Create fragment shader object
    let frag_shader = gl.create_shader(WebGlRenderingContext::FRAGMENT_SHADER)
        .unwrap();

    // Attach fragment shader source code
    gl.shader_source(&frag_shader, frag_code);

    // Compile the fragmentt shader
    gl.compile_shader(&frag_shader);

    // Create a shader program object to
    // store the combined shader program
    let shader_program = gl.create_program()
        .unwrap();

    // Attach a vertex shader
    gl.attach_shader(&shader_program, &vert_shader);

    // Attach a fragment shader
    gl.attach_shader(&shader_program, &frag_shader);

    // Link both the programs
    gl.link_program(&shader_program);

    // Use the combined shader program object
    gl.use_program(Some(&shader_program));

    /* ======== Associating shaders to buffer objects =======*/

    // Bind vertex buffer object
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

    // Bind index buffer object
    gl.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));

    // Get the attribute location
    let coord = gl.get_attrib_location(&shader_program, "coordinates");

    // point an attribute to the currently bound VBO
    gl.vertex_attrib_pointer_with_f64(coord as u32, 3, WebGlRenderingContext::FLOAT, false, 0, 0.0);

    // Enable the attribute
    gl.enable_vertex_attrib_array(coord as u32);

    // bind the color buffer
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&color_buffer));
    
    // get the attribute location
    let color = gl.get_attrib_location(&shader_program, "color");

    // point attribute to the volor buffer object
    gl.vertex_attrib_pointer_with_f64(color as u32, 3, WebGlRenderingContext::FLOAT, false, 0, 0.0) ;

    // enable the color attribute
    gl.enable_vertex_attrib_array(color as u32);

    /*============Drawing the Quad====================*/

    // Clear the canvas
    gl.clear_color(0.5, 0.5, 0.5, 0.9);

    // Enable the depth test
    gl.enable(WebGlRenderingContext::DEPTH_TEST);

    // Clear the color buffer bit
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    // Set the view port
    gl.viewport(0, 0, canvas.width().try_into().unwrap(),canvas.height().try_into().unwrap());

    //Draw the triangle
    gl.draw_elements_with_f64(WebGlRenderingContext::TRIANGLES, indices.len().try_into().unwrap(), WebGlRenderingContext::UNSIGNED_SHORT, 0.0);

    Ok(gl)
}