extern crate sdl2;
extern crate gl;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use gl::types::*;


fn main()
{
    let context = sdl2::init().unwrap();
    let video_context = context.video().unwrap();
    let mut timer = context.timer().unwrap();

    let mut window = match video_context.window("SDL2", 800, 600).position_centered().opengl().build(){
        Ok(window) => window,
        Err(e) => panic!("Failed to create window: {}", e)
    };

    let gl_context = window.gl_create_context().unwrap();
    let gl = gl::load_with(|s| video_context.gl_get_proc_address(s) as *const std::os::raw::c_void);
    let gl_attribute = video_context.gl_attr();
    gl_attribute.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attribute.set_context_version(4, 5);

    //display
    window.show();

    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    //events
    let mut events = context.event_pump().unwrap();
    'event : loop {

        for event in events.poll_iter(){
            match event{
                Event::Quit {..} => break 'event,
                _ => continue
            }

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            window.gl_swap_window();
        }
    }
}

fn shader_from_source(source: &str, kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };

}