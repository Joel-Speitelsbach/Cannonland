
use battlefield;
use present;
use sdl2;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::rect::Rect;


pub fn run() {

    // init window
    let win_size: (i32,i32) = battlefield::SIZE; 
    let sdl_context = sdl2::init().unwrap();
    let mut canvas = present::new_window(&sdl_context.video().unwrap(), win_size);

    // create fps manager
    let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
    fps_manager.set_framerate(20).unwrap();


    // create ttf
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font("pics/LiberationSans.ttf",60).unwrap();
    let texture_creator = canvas.texture_creator();

    let surface = 
            font
            .render("my Text to be printed")
            .blended(sdl2::pixels::Color::RGB(200,200,200))
            .unwrap();
    let height = surface.height();
    let width = surface.width();
    let mut  texture = texture_creator.create_texture_from_surface(surface).unwrap();
    texture.set_blend_mode(sdl2::render::BlendMode::Blend);


    'mainloop: loop {

        canvas.clear();
        canvas.copy(&texture, None, Rect::new(200, 200, width, height)).unwrap();

        canvas.present();

        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Right),.. } => break 'mainloop,
                _ => {},
            }
        }

        fps_manager.delay();
    }
}