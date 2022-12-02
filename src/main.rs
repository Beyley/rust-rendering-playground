use glfw::{Action, Context, Key};
use std::ptr;
use std::ffi::c_void;
use std::ffi::CStr;
use std::mem::size_of;

fn main() {
    //Init GLFW
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize GLFW!");

    //Hint to use GL 3.1 core
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(2));
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGl));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    //Create the window
    let (mut window, events) = glfw.create_window(300, 300, "Hello Window", glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");

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

    //Create shaders
    let vtxShader = rgl::shaders::create_shader(rgl::ShaderType::Vertex);
    let frgShader = rgl::shaders::create_shader(rgl::ShaderType::Fragment);

    //Set the source code of the shaders
    rgl::shaders::shader_source(vtxShader, "#version 330 core
layout (location = 0) in vec3 aPos;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
");
    rgl::shaders::shader_source(frgShader, "#version 330 core
out vec4 FragColor;
void main()
{
    FragColor = vec4(1.0f, 0.0f, 0.0f, 1.0f);
}
");

    //Compile the shaders
    rgl::shaders::compile_shader(vtxShader);
    rgl::shaders::compile_shader(frgShader);

    //Create the shader program
    let program = rgl::shaders::create_program();

    //Attach both shaders to the program
    rgl::shaders::attach_shader(program, vtxShader);
    rgl::shaders::attach_shader(program, frgShader);

    //Link the program
    rgl::shaders::link_program(program);

    //Delete the shaders
    rgl::shaders::delete_shader(vtxShader);
    rgl::shaders::delete_shader(frgShader);

    let vao = rgl::buffers::gen_vertex_array();
    let vbo = rgl::buffers::gen_buffer();

    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0,  0.5, 0.0
    ];

    rgl::buffers::bind_vertex_array(vao);
    rgl::buffers::bind_buffer(rgl::Target::ArrayBuffer, vbo);
    rgl::buffers::buffer_data(rgl::Target::ArrayBuffer, &vertices, rgl::Usage::StaticDraw);
    rgl::buffers::vertex_attrib_pointer(0, 3, rgl::Type::Float, false, (size_of::<f32>() * 3) as i32);
    rgl::buffers::enable_vertex_attrib_array(0);

    //Enable VSync
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        //Set the clear color to 0, 0, 0, 1
        rgl::drawing::clear_color(0.0, 0.0, 0.0, 1.0);
        unsafe {
            //Clear the color bit of the framebuffer
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        rgl::shaders::use_program(program);
        rgl::buffers::bind_vertex_array(vao);
        rgl::drawing::draw_arrays(rgl::Primitive::Triangles, 0, 3);

        window.swap_buffers();
    }
}

extern "system" fn gl_debug_callback(_source: u32, _error_type: u32, _id: u32, _severity: u32, _length: i32, message_ptr: *const i8, _user_param: *mut c_void) {
    unsafe {
        let message = CStr::from_ptr(message_ptr);

        println!("GL Callback {}", message.to_str().unwrap());
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        glfw::WindowEvent::FramebufferSize(width, height) => {
            unsafe {
                println!("Changing framebuffer size to {}x{}", width, height);
                gl::Viewport(0, 0, width, height);
            }
        }
        _ => {}
    }
}