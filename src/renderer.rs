use std::ffi::CStr;

pub struct Renderer {
    pub program: Option<rgl::Program>
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            program: None
        }
    }

    pub fn create_program(&mut self) {
        if let Some(x) = self.program {
            rgl::shaders::delete_program(x);
            self.program = None;
        }

        //Create shaders
        let vtx_shader = rgl::shaders::create_shader(rgl::ShaderType::Vertex);
        let frg_shader = rgl::shaders::create_shader(rgl::ShaderType::Fragment);

        //Set the source code of the shaders
        rgl::shaders::shader_source(vtx_shader, "#version 330 core
        layout (location = 0) in vec2 aPos;
        layout (location = 1) in vec2 vtxTexCoord;

        out vec2 texCoord;

        uniform mat3x2 projectionMatrix;

        void main()
        {
        gl_Position = vec4(projectionMatrix * vec3(aPos, 1.0), 0.0, 1.0);
        texCoord = vtxTexCoord;
        }
        ");
        rgl::shaders::shader_source(frg_shader, "#version 330 core

        in vec2 texCoord;

        out vec4 FragColor;

        uniform sampler2D textureSampler;

        void main()
        {
        FragColor = texture(textureSampler, texCoord);
        }
        ");

        //Compile the shaders
        rgl::shaders::compile_shader(vtx_shader);
        rgl::shaders::compile_shader(frg_shader);

        //Create the shader program
        self.program = Some(rgl::shaders::create_program());

        //Attach both shaders to the program
        rgl::shaders::attach_shader(self.program.unwrap(), vtx_shader);
        rgl::shaders::attach_shader(self.program.unwrap(), frg_shader);

        //Link the program
        rgl::shaders::link_program(self.program.unwrap());

        unsafe {
            let mut link_status = 0;

            gl::GetProgramiv(*(&self.program.unwrap() as *const _ as *const u32), gl::LINK_STATUS, &mut link_status);

            println!("Program link status: {}", link_status);

            if link_status == 0 {
                let mut info_log_length = 0;
                let mut log: [u8; 1000] = [0; 1000];

                gl::GetProgramInfoLog(*(&self.program as *const _ as *const u32), log.len() as i32, &mut info_log_length, log.as_mut_ptr() as *mut _);

                let str = CStr::from_bytes_with_nul(&log).expect("wat");

                println!("Program info log: {}", str.to_str().unwrap());
                panic!("Failed to link program");
            }
        }

        println!("Linking shader");

        unsafe {
            gl::DetachShader(*(&self.program as *const _ as *const u32), *(&vtx_shader as *const _ as *const u32));
            gl::DetachShader(*(&self.program as *const _ as *const u32), *(&frg_shader as *const _ as *const u32));
        }

        //Delete the shaders
        rgl::shaders::delete_shader(vtx_shader);
        rgl::shaders::delete_shader(frg_shader);
    }
}