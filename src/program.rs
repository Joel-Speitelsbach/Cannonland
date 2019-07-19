use battlefield;
use client;
use present;
use sdl2;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::render::Canvas;


pub struct Window {
    pub sdl_context: sdl2::Sdl,
    pub canvas: Canvas<sdl2::video::Window>,
}


pub fn run() {

    // init window
    let win_size: (i32,i32) = battlefield::SIZE; 
    let sdl_context = sdl2::init().unwrap();
    let canvas = present::new_window(&sdl_context.video().unwrap(), win_size);

    // create fps manager
    let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
    fps_manager.set_framerate(20).unwrap();

    'mainloop: loop {
        /*

        //// UNTERMENÜS
        if mauszeiger_click() {
            client::run();
        }
        if button_für_server() 
            && server_nicht_gestartet 
        {
            als_hintergrundprozess(server::run);
        }

        zeichne_hauptmenü();
        screen.blit_surface();

        */

        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Right),.. } => break 'mainloop,
                _ => {},
            }
        }

        fps_manager.delay();
    }

    let mut window = Window {
        sdl_context,
        canvas,
    };
    client::run("localhost", &mut window);
}
