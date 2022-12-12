use glfw::{Action, Context, Key};
use std::ptr;
use std::ffi::c_void;
use std::ffi::CStr;
use std::mem::size_of;

mod texture;
mod renderer;

use texture::load_texture;

fn glfw_callback(err: glfw::Error, message: String, _: &()) {
    println!("GLFW Error {}: {}", err, message);
}

fn main() {
    //Init GLFW
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize GLFW");

    //Hint to use GL 3.2 core
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(2));
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGl));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    glfw.set_error_callback(Some(glfw::Callback {
        f: glfw_callback,
        data: ()
    }));

    //Create the window
    let (mut window, events) = glfw.create_window(300, 300, "Hello Window", glfw::WindowMode::Windowed).expect("Failed to create GLFW window");

    //Enable key event polling
    window.set_key_polling(true);
    //Enable framebuffer size event polling
    window.set_framebuffer_size_polling(true);
    //Make the OpenGL context current
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    #[cfg(debug_assertions)]
    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);

        gl::DebugMessageCallback(gl_debug_callback, ptr::null());
    }

    let mut renderer = renderer::Renderer::new();

    renderer.create_program();

    //Generate a vertex array and vertex buffer
    let vao = rgl::buffers::gen_vertex_array();
    let vbo = rgl::buffers::gen_buffer();
    let ebo = rgl::buffers::gen_buffer();

    let size: f32 = 100.0;
    let vertices: &[f32] = &[
        0.0,  0.0,  0.0, 0.0, //bottom left
        0.0,  size, 0.0, 1.0, //top left
        size, 0.0,  1.0, 0.0, //bottom right
        size, size, 1.0, 1.0  //top right
    ];

    let indices: &[u16] = &[
        0, 1, 2,
        3, 1, 2
    ];

    rgl::buffers::bind_vertex_array(vao);
    rgl::buffers::bind_buffer(rgl::Target::ArrayBuffer, vbo);
    rgl::buffers::buffer_data(rgl::Target::ArrayBuffer, &vertices, rgl::Usage::StaticDraw);
    rgl::buffers::bind_buffer(rgl::Target::ElementArrayBuffer, ebo);
    rgl::buffers::buffer_data(rgl::Target::ElementArrayBuffer, &indices, rgl::Usage::StaticDraw);
    unsafe {
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 4) as i32, 0 as *const c_void);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 4) as i32, (size_of::<f32>() * 2) as *const c_void);
    }
    rgl::buffers::enable_vertex_attrib_array(0);
    rgl::buffers::enable_vertex_attrib_array(1);

    //Load texture
    let tex = load_texture("texture.png");

    //Enable VSync
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    rgl::shaders::use_program(renderer.program.unwrap());

    //Reset the time to 0
    glfw.set_time(0.0);
    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event, renderer.program.unwrap());
        }

        //Get the current time of the program
        //let time_s: f64 = glfw.get_time();

        //Set the clear color to 0, 0, 0, 1
        rgl::drawing::clear_color(0.0, 0.0, 0.0, 1.0);
        unsafe {
            //Clear the color bit of the framebuffer
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        rgl::buffers::bind_vertex_array(vao);
        rgl::textures::bind_texture(rgl::TexTarget::_2D, tex);
        rgl::drawing::draw_elements(rgl::Primitive::Triangles, 6, rgl::Type::UShort);

        window.swap_buffers();
    }
}

extern "system" fn gl_debug_callback(_source: u32, _error_type: u32, _id: u32, _severity: u32, _length: i32, message_ptr: *const i8, _user_param: *mut c_void) {
    unsafe {
        let message = CStr::from_ptr(message_ptr);

        println!("GL Callback {}", message.to_str().unwrap());
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent, program: rgl::Program) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        glfw::WindowEvent::FramebufferSize(width, height) => {
            unsafe {
                println!("Changing framebuffer size to {}x{}", width, height);
                gl::Viewport(0, 0, width, height);

                update_projection_matrix(program, width as f32, height as f32);
            }
        }
        _ => {}
    }
}

fn update_projection_matrix(program: rgl::Program, width: f32, height: f32) {
    let matrix = [
            [2.0 / width, 0.0, -1.0], // Row 0
            [0.0, -2.0 / height, 1.0], // Row 1
        ];

    let uniform_location = rgl::get_uniform_location(program, "projectionMatrix");


    unsafe {
        println!("Updating proj matrix uniform:{}", *(&uniform_location as *const _ as *const i32));
        gl::UniformMatrix3x2fv(*(&uniform_location as *const _ as *const i32), 1, gl::TRUE, matrix.as_ptr() as *const _);
    }
}