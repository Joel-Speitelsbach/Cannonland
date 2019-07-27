use sdl2::video::WindowContext;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use battlefield;
use client;
use present;
use sdl2;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::rect::Rect;


pub struct Window {
    pub sdl_context: sdl2::Sdl,
    pub canvas: Canvas<sdl2::video::Window>,
}


pub fn run() {

    // init window
    let win_size: (i32,i32) = battlefield::SIZE;
    let sdl_context = sdl2::init().unwrap();
    let mut canvas = present::new_window(&sdl_context.video().unwrap(), win_size);

    // create fps manager
    let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
    fps_manager.set_framerate(20).unwrap();

    let texture_creator = canvas.texture_creator();
    let game_name_texture = create_text(&texture_creator, "CannonLand");
    let host_texture = create_text(&texture_creator, "Host");
    let join_texture = create_text(&texture_creator, "Join");

    let mut selected_index : u8 = 0;

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

        let mut host_extra_width: u32= 0;
        let mut join_extra_width: u32 = 0;
        if selected_index == 0 {
            host_extra_width = 200;
        } else {
            join_extra_width = 200;
        }

        canvas.clear();
        canvas.copy(&game_name_texture, None, Rect::new(400, 100, game_name_texture.query().width, game_name_texture.query().height)).unwrap();
        canvas.copy(&host_texture, None, Rect::new(600 - host_extra_width as i32/2, 400, host_texture.query().width + host_extra_width, host_texture.query().height)).unwrap();
        canvas.copy(&join_texture, None, Rect::new(600 - join_extra_width as i32/2, 600, join_texture.query().width + join_extra_width, join_texture.query().height)).unwrap();
        canvas.present();

        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Right),.. } => break 'mainloop,
                Event::KeyUp { keycode: Some(Keycode::Up),.. } => selected_index = (selected_index+3)%2,
                Event::KeyUp { keycode: Some(Keycode::Down),.. } => selected_index = (selected_index+1)%2,
                Event::Quit{..} => break 'mainloop,
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

fn create_text<'t> (texture_creator: &'t TextureCreator<WindowContext>, text: &str) -> Texture<'t> {
    // create ttf
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font("pics/LiberationSans.ttf",120).unwrap();

    let surface =
            font
            .render(&text)
            .blended(sdl2::pixels::Color::RGB(200,200,200))
            .unwrap();
    let mut  texture = texture_creator.create_texture_from_surface(surface).unwrap();
    texture.set_blend_mode(sdl2::render::BlendMode::Blend);
    return texture;
}
