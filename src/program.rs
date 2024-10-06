use std::thread;
use std::time::Duration;

use sdl2::video::WindowContext;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use crate::battlefield;
use crate::client;
use crate::present;
use crate::serverless_client;
use crate::server;
use sdl2;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use crate::sound::Sound;


pub struct Window {
    pub sdl_context: sdl2::Sdl,
    pub canvas: Canvas<sdl2::video::Window>,
    pub sound: Sound,
}

pub fn run() {

    // init window
    let win_size: (i32,i32) = battlefield::SIZE;
    let sdl_context = sdl2::init().unwrap();
    let canvas = present::new_window(&sdl_context.video().unwrap(), win_size);
    let mut window = Window {
        sdl_context,
        canvas,
        sound: Sound::init(),
    };


    // create fps manager
    let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
    fps_manager.set_framerate(20).unwrap();

    let texture_creator = window.canvas.texture_creator();
    let game_name_texture = create_text(&texture_creator, "Cannonland");
    let mut host_texture = create_text(&texture_creator, "Host");
    let join_texture = create_text(&texture_creator, "Join");
    let single_texture = create_text(&texture_creator, "Singleplayer");

    let mut selected_index : u8 = 0;
    let mut server_started = false;

    'mainloop: loop {
        /*

        //// UNTERMENÜS
        if button_für_server()
            && server_nicht_gestartet
        {
            als_hintergrundprozess(server::run);
        }
        */

        let mut host_extra_width: u32= 0;
        let mut join_extra_width: u32 = 0;
        let mut single_extra_width: u32 = 0;
        if selected_index == 0 {
            host_extra_width = 200;
        } else if selected_index == 1 {
            join_extra_width = 200;
        } else {
            single_extra_width = 200;
        }

        window.canvas.set_draw_color(Color::RGB(0, 0, 0));
        window.canvas.clear();
        window.canvas.copy(&game_name_texture, None, Rect::new(400, 0, game_name_texture.query().width, game_name_texture.query().height)).unwrap();
        window.canvas.copy(&host_texture, None, Rect::new(600 - host_extra_width as i32/2, 250, host_texture.query().width + host_extra_width, host_texture.query().height)).unwrap();
        window.canvas.copy(&join_texture, None, Rect::new(600 - join_extra_width as i32/2, 450, join_texture.query().width + join_extra_width, join_texture.query().height)).unwrap();
        window.canvas.copy(&single_texture, None, Rect::new(400 - single_extra_width as i32/2, 650, single_texture.query().width + single_extra_width, join_texture.query().height)).unwrap();
        window.canvas.present();

        // event loop
        loop {
            let event = match window.sdl_context.event_pump().unwrap().poll_event() {
                Some(event) => event,
                None => {
                    break;
                },
            };
            match event {
                Event::KeyDown { keycode: Some(Keycode::Right | Keycode::Return),.. } => {
                    if selected_index == 0 {
                        if !server_started {
                            thread::spawn(|| server::run());
                            host_texture = create_text(&texture_creator, "Host (started)");
                            server_started = true;
                            thread::sleep(Duration::from_millis(500));
                        }
                        client::run("localhost", &mut window);
                    } else if selected_index == 1 {
                        run_client_menu(&mut window);
                    } else if selected_index == 2 {
                        serverless_client::run(&mut window);
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::Up),.. } => selected_index = (selected_index+2)%3,
                Event::KeyUp { keycode: Some(Keycode::Down),.. } => selected_index = (selected_index+1)%3,
                Event::Quit{..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => break 'mainloop,
                _ => {},
            }
        }

        fps_manager.delay();
    }
}


fn run_client_menu(window: &mut Window) {
    let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
    fps_manager.set_framerate(20).unwrap();

    let texture_creator = window.canvas.texture_creator();
    let mut server_address = "localhost".to_string();
    'mainloop: loop {
        let address_line = "server: ".to_string() + &server_address;
        let address_texture = create_text(&texture_creator, &address_line);
        window.canvas.set_draw_color(Color::RGB(0, 0, 0));
        window.canvas.clear();
        window.canvas.copy(&address_texture, None, Rect::new(200, 300, address_texture.query().width, address_texture.query().height)).unwrap();
        window.canvas.present();
        
        // event loop
        loop {
            let event = match window.sdl_context.event_pump().unwrap().poll_event() {
                Some(event) => event,
                None => {
                    break;
                },
            };
            match event {
                Event::TextInput { text, ..} => {
                    // println!("text: {text}");
                    server_address.push_str(&text);
                },
                Event::KeyDown { keycode: Some(Keycode::Backspace), ..} => {
                    let mut chars = server_address.chars();
                    chars.next_back();
                    server_address = chars.as_str().to_string();
                },
                Event::KeyDown { keycode: Some(Keycode::Return), ..} => {
                    client::run(&server_address, window);
                }
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => break 'mainloop,
                Event::Quit {..} => std::process::exit(0),
                _ => {}
            }
        }

        fps_manager.delay();
    }
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
    let mut texture = texture_creator.create_texture_from_surface(surface).unwrap();
    texture.set_blend_mode(sdl2::render::BlendMode::Blend);
    return texture;
}
