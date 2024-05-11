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
pub fn draw_triangle() -> Result<WebGlRenderingContext, JsValue> {

    let canvas = get_canvas("wasm_canvas");

    /* Step1: Prepare the canvas and get WebGL context */
    let gl = canvas
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap();

    /* Step2: Define the geometry and store it in buffer objects */
    let vertices: [f32; 6] = [-0.5, 0.5, -0.5, -0.5, 0.0, -0.5];
    let vertices_array = unsafe { js_sys::Float32Array::view(&vertices) };

    // Create a new buffer object
    let vertex_buffer = gl.create_buffer().unwrap();

    // Bind an empty array buffer to it
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

    // Pass the vertices data to the buffer
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &vertices_array,
        WebGlRenderingContext::STATIC_DRAW
    );

    // Unbind the buffer
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);

    /* Step3: Create and compile Shader programs */
    let vert_code ="
        attribute vec2 coordinates;
        void main(void) {
            gl_Position = vec4(coordinates,0.0, 1.0);
        }
    ";

    //Create a vertex shader object
    let vert_shader = gl.create_shader(WebGlRenderingContext::VERTEX_SHADER)
        .ok_or_else(|| JsValue::from_str("create vert shader error"))?;

    //Attach vertex shader source code
    gl.shader_source(&vert_shader, vert_code);

    //Compile the vertex shader
    gl.compile_shader(&vert_shader);

    //Fragment shader source code
    let frag_code = "
        void main(void) {
            gl_FragColor = vec4(0.0, 0.0, 0.0, 0.1);
        }
    ";

    // Create fragment shader object
    let frag_shader = gl.create_shader(WebGlRenderingContext::FRAGMENT_SHADER)
        .ok_or_else(|| JsValue::from_str("create frag shader error"))?;

    // Attach fragment shader source code
    gl.shader_source(&frag_shader, frag_code);

    // Compile the fragment shader
    gl.compile_shader(&frag_shader);

    // Create a shader program object to store combined shader program
    let shader_program = gl.create_program().unwrap();

    // Attach a vertex shader
    gl.attach_shader(&shader_program, &vert_shader);

    // Attach a fragment shader
    gl.attach_shader(&shader_program, &frag_shader);

    // Link both programs
    gl.link_program(&shader_program);

    // Use the combined shader program object
    gl.use_program(Some(&shader_program));

    /* Step 4: Associate the shader programs to buffer objects */

    //Bind vertex buffer object
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

    //Get the attribute location
    let coord = gl.get_attrib_location(&shader_program, "coordinates");

    //point an attribute to the currently bound VBO
    gl.vertex_attrib_pointer_with_i32(coord as u32, 2, WebGlRenderingContext::FLOAT, false, 0, 0);

    //Enable the attribute
    gl.enable_vertex_attrib_array(coord as u32);

    /* Step5: Drawing the required object (triangle) */

    // Clear the canvas
    gl.clear_color(0.5, 0.5, 0.5, 0.9);

    // Enable the depth test
    gl.enable(WebGlRenderingContext::DEPTH_TEST); 
         
    // Clear the color buffer bit
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    // Set the view port
    gl.viewport(0, 0, canvas.width().try_into().unwrap(), canvas.height().try_into().unwrap());

    // Draw the triangle
    gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 3);

    Ok(gl)
}