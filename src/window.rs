
use std::path::Path;
use sdl2;
use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;

use data::cell::Cell;

pub fn run(png: &Path) {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();
    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
      .position_centered()
      .build()
      .unwrap();

    let mut renderer = window.renderer().software().build().unwrap();
    let texture = renderer.load_texture(png).expect("Loading image failed");

    renderer.copy(&texture, None, None).expect("Render failed");

    let cell = Cell::new(100 as f32);
    renderer.circle(100, 100, cell.radius as i16, (255, 255, 255, 255)).unwrap();

    renderer.present();

    'mainloop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {}
            }
        }
    }
}
