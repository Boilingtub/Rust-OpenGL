extern crate gl;
extern crate half;
extern crate sdl2;
extern crate vec_2_10_10_10;
#[macro_use] extern crate failure;
#[macro_use] extern crate render_gl_derive as render_gl_derive;

mod debug;
pub mod render_gl;
pub mod resources;

use failure::err_msg;
use crate::resources::Resources;
use std::path::Path;

mod triangle;

#[derive(VertexAttribPointers ,Copy , Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = "0"]
    pos: render_gl::data::f32_f32_f32,
    #[location = "1"]
    clr: render_gl::data::u2_u10_u10_u10_rev_float,
}

fn main() {
    println!("<>><<>><<>><<><>><<>><");
    println!("Rusty-GL -0.1.0- ; SDL2 ");
    println!("<>><<>><<>><<>><<>><<>\n\n\n");

    if let Err(e) = run() {
        println!("{}", debug::failure_to_string(e));
    }

}

fn run() -> Result<(), failure::Error> {
   
    const DEFAULT_WINDOW_WIDTH: u16 = 720;
    const DEFAULT_WINDOW_HEIGHT: u16 = 400;

    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

    let sdl = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl.video().map_err(err_msg)?;

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Game", DEFAULT_WINDOW_WIDTH.into(), DEFAULT_WINDOW_HEIGHT.into())
        .opengl()
        .resizable()
        .build()?;

    let _gl_context = window.gl_create_context().map_err(err_msg)?;
    let gl = gl::Gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    let mut viewport = render_gl::Viewport::for_window(DEFAULT_WINDOW_WIDTH.into(), DEFAULT_WINDOW_HEIGHT.into());
    
    let color_buffer = render_gl::ColorBuffer::from_color(1.0, 1.0, 1.0, 1.0);
    let triangle = triangle::Triangle::new(&res, &gl)?;

    // setup shared state for window
    
    viewport.set_used(&gl);
    color_buffer.set_used(&gl);

    // main loop

    let mut event_pump = sdl.event_pump().map_err(err_msg)?;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {

                sdl2::event::Event::Quit { .. } => break 'main,

                sdl2::event::Event::Window { win_event: sdl2::event::WindowEvent::Resized(w, h),..}
                    => {
                        viewport.update_size(w, h);
                        viewport.set_used(&gl)
                    },

                _ => {}
            }
        }

        color_buffer.clear(&gl);
        triangle.render(&gl);

        window.gl_swap_window();
    }

    Ok(())
}









