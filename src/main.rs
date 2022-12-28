extern crate gl;
extern crate sdl2;

pub mod render_gl;

fn main() {
    println!("<>><<>><<>><<><>><<>><");
    println!("Rusty-GL -0.1.0- ; SDL2 ");
    println!("<>><<>><<>><<>><<>><<>\n\n\n");

    const WINDOW_WIDTH:u16 = 900;
    const WINDOW_HEIGHT:u16 = 700;


    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);  

    let window = video_subsystem
        .window("Rusty-GL" , WINDOW_WIDTH.into() , WINDOW_HEIGHT.into())
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void});

    use std::ffi::CString;

    let vert_shader = render_gl::Shader::from_vert_source(
        &gl, &CString::new(include_str!("triangle.vert")).unwrap()
    ).unwrap();

    let frag_shader = render_gl::Shader::from_frag_source(
        &gl, &CString::new(include_str!("triangle.frag")).unwrap()
    ).unwrap();

    let shader_program = render_gl::Program::from_shaders(
        &gl, &[vert_shader, frag_shader]
    ).unwrap();    

    //setup vertex Buffer object

    let vertices: Vec<f32> = vec![
        //positions         //colors
        -0.5, -0.5, 0.0,    1.0, 0.0, 0.0,
        0.5, -0.5, 0.0,     0.0,1.0,0.0,
        0.0, 0.5, 0.0,       0.0,0.0,1.0
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl.GenBuffers(1, &mut vbo);
    }
    
    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * std::mem::size_of::<f32>())
                       as gl::types::GLsizeiptr,
                       vertices.as_ptr() as *const gl::types::GLvoid,
                       gl::STATIC_DRAW, 
                       );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
                        
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl.GenVertexArrays(1, &mut vao);
    }


    unsafe {
        gl.BindVertexArray(vao);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl.EnableVertexAttribArray(0);
        gl.VertexAttribPointer(0,
                                3,
                                gl::FLOAT,
                                gl::FALSE,
                                (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
                                std::ptr::null(),  );

        gl.EnableVertexAttribArray(1);
        gl.VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
            );

        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
    }

    //set up shared state for window

    unsafe {
        gl.Viewport(0, 0, WINDOW_WIDTH.into(), WINDOW_HEIGHT.into() );
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }


        //MAIN loop

    let mut event_pump = sdl.event_pump().unwrap();    
   'main: loop {

        for event in event_pump.poll_iter() {
            //Handel user input      
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        //handel Window contents
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }
    
        // draw triangle
        

        shader_program.set_used();
        unsafe {
            gl.BindVertexArray(vao);
            gl.DrawArrays(gl::TRIANGLES,
                       0,
                       3 );
        }


        window.gl_swap_window();

    }
}











