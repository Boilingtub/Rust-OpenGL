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
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);


    unsafe {
        gl::Viewport(0, 0, WINDOW_WIDTH.into(), WINDOW_HEIGHT.into() );
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }


    let mut event_pump = sdl.event_pump().unwrap();    

    use std::ffi::CString;

    let vert_shader = render_gl::Shader::from_vert_source(
        &CString::new(include_str!("triangle.vert")).unwrap()
    ).unwrap();

    let frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(include_str!("triangle.frag")).unwrap()
    ).unwrap();

    let shader_program = render_gl::Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();

    shader_program.set_used();

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
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();

    }
}











