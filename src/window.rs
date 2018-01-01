use std::time::SystemTime;

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::video::Window;
use sdl2::pixels;

use grid;
use grid::color::Color;

pub fn run() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Rost", 1600, 1000)
      .position_centered()
      .build()
      .unwrap();

    let mut game = Game::new(window.into_canvas().build().unwrap());

    'mainloop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {}
            }
        }
        game.stride();
    }
}

struct Game {
    canvas: sdl2::render::Canvas<Window>,
    fps_manager: sdl2::gfx::framerate::FPSManager,
    particle_size: i16,
    grid: grid::Grid
}

impl Game {
    fn new(canvas: sdl2::render::Canvas<Window>) -> Game {
        let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
        fps_manager.set_framerate(60).unwrap();
        Game {
            canvas: canvas,
            fps_manager: fps_manager,
            particle_size: 2,
            grid: grid::create_grid()
        }
    }

    fn stride(&mut self) -> () {

        let calc_time = SystemTime::now();
        self.grid.stride();
        print!("calc needed {} msecs", calc_time.elapsed().unwrap().subsec_nanos() / (1000*1000));

        let present_time = SystemTime::now();
        self.present_grid();
        print!(", present needed {} msecs", present_time.elapsed().unwrap().subsec_nanos() / (1000*1000));

        println!(", calc and present needed {} msecs", calc_time.elapsed().unwrap().subsec_nanos() / (1000*1000));

        self.fps_manager.delay();
    }

    fn present_grid(&mut self) {
        self.canvas.set_draw_color(pixels::Color::RGBA(96,128,200,255));
        self.canvas.clear();

        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                self.draw_particle(x, y);
            }
        }

        self.canvas.present();
    }

    fn draw_particle(&mut self, x: usize, y:usize) -> () {
        match self.grid.grid[y][x].color {
            Color::EMPTY => return,
            Color::BLUR => return,
            Color::DIRT => self.draw_color(x, y, pixels::Color::RGBA(128,64,0,255)),
            Color::ROCK => self.draw_color(x, y, pixels::Color::RGBA(128,128,128,255)),
            Color::BETON => self.draw_color(x, y, pixels::Color::RGBA(194,194,194,255)),
            Color::SNOW => self.draw_color(x, y, pixels::Color::RGBA(255,255,255,255)),
            Color::WATER => self.draw_color(x, y, pixels::Color::RGBA(0,0,200,255))
        }
    }

    fn draw_color(&mut self, x: usize, y: usize, rgba: pixels::Color) -> () {
        let x_scaled = x as i16 * self.particle_size;
        let y_scaled = y as i16 * self.particle_size;
        self.canvas.box_(x_scaled, y_scaled, x_scaled+self.particle_size, y_scaled+self.particle_size, rgba).unwrap();
    }

}
