
use std::path::Path;
use sdl2;
use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color::RGB;
use sdl2::pixels::Color::RGBA;

// use data::cell::Cell;

pub fn run(png: &Path) {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();
    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
      .position_centered()
      .build()
      .unwrap();

    let mut game = Game::new(window.renderer().software().build().unwrap());

    'mainloop: loop {
        game.present();
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {}
            }
        }
        game.iter();
    }
}

struct Game<'a> {
    renderer: sdl2::render::Renderer<'a>,
    fps_manager: sdl2::gfx::framerate::FPSManager,
    x: i16,
    y: i16,
}

impl<'a> Game<'a> {
    fn new(renderer: sdl2::render::Renderer<'a>) -> Game<'a> {
        let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
        fps_manager.set_framerate(60).unwrap();
        Game {
            renderer: renderer,
            fps_manager: fps_manager,
            x: 200,
            y: 200,
        }
    }
    fn present(&mut self) {
        self.renderer.set_draw_color(RGBA(0,0,0,255));
        self.renderer.clear();
        self.renderer.circle(self.x, self.y, 100 as i16, (255, 255, 255, 255)).unwrap();
        self.fps_manager.delay();
        self.renderer.present();
    }
    fn iter(&mut self) {
        self.x += 1;
        self.x %= 300;
    }
}