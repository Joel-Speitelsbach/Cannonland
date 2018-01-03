use std::time::SystemTime;

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::video::Window;
use sdl2::pixels;

use game::grid;
use game;

pub fn run() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Rost", 1600, 1000)
      .position_centered()
      .build()
      .unwrap();

    let mut presenter = Presenter::new(window.into_canvas().build().unwrap());

    'mainloop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {}
            }
        }
        presenter.stride();
    }
}

struct Presenter {
    game: game::Game,
    grid_presenter: GridPresenter,
    fps_manager: sdl2::gfx::framerate::FPSManager
}

impl Presenter {

    pub fn new(canvas: sdl2::render::Canvas<Window>) -> Presenter {
        let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
        fps_manager.set_framerate(60).unwrap();
        return Presenter{
            game: game::Game::new(),
            grid_presenter: GridPresenter::new(canvas),
            fps_manager: fps_manager
        };
    }

    fn stride(&mut self) -> () {

        let calc_time = SystemTime::now();
        self.grid_presenter.grid.stride();
        print!("calc needed {} msecs", calc_time.elapsed().unwrap().subsec_nanos() / (1000*1000));

        let present_time = SystemTime::now();
        self.grid_presenter.present();
        print!(", present needed {} msecs", present_time.elapsed().unwrap().subsec_nanos() / (1000*1000));

        println!(", calc and present needed {} msecs", calc_time.elapsed().unwrap().subsec_nanos() / (1000*1000));

        self.fps_manager.delay();
    }

}

struct GridPresenter {
    canvas: sdl2::render::Canvas<Window>,
    particle_size: i16,
    grid: grid::Grid
}

impl GridPresenter {
    fn new(canvas: sdl2::render::Canvas<Window>) -> GridPresenter {
        GridPresenter {
            canvas: canvas,
            particle_size: 2,
            grid: grid::create_grid()
        }
    }

    fn present(&mut self) -> () {
        self.draw_background();

        self.draw_particles();
        self.draw_bunkers();

        self.canvas.present();
    }

    fn draw_background(&mut self) -> () {
        self.canvas.set_draw_color(pixels::Color::RGBA(96,128,200,255));
        self.canvas.clear();
    }

    fn draw_particles(&mut self) -> () {
        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                self.draw_particle(x, y);
            }
        }
    }

    fn draw_particle(&mut self, x: usize, y: usize) -> () {
        let rgba: (u8,u8,u8,u8) = self.grid.grid[y][x].get_rgba();

        if rgba.3 != 0 {
            let color = pixels::Color::RGBA(rgba.0, rgba.1, rgba.2, rgba.3);
            let x_scaled = x as i16 * self.particle_size;
            let y_scaled = y as i16 * self.particle_size;
            self.canvas.box_(x_scaled, y_scaled, x_scaled+self.particle_size, y_scaled+self.particle_size, color).unwrap();
        }
    }

    fn draw_bunkers(&mut self) -> () {
        for bunker in &self.grid.bunkers {
            let cannon_pos: (i16,i16,i16,i16) = bunker.1.get_cannon_pos_x1y1x2y2();
            let x_scaled = bunker.1.x_pos * self.particle_size;
            let y_scaled = bunker.1.y_pos * self.particle_size;
            let rgba: (u8,u8,u8,u8) = bunker.0.get_rgba();
            let color = pixels::Color::RGBA(rgba.0, rgba.1, rgba.2, rgba.3);

            self.canvas.thick_line(
                cannon_pos.0 * self.particle_size, cannon_pos.1 * self.particle_size,
                cannon_pos.2 * self.particle_size, cannon_pos.3 * self.particle_size,
                2, color).unwrap();
            self.canvas.filled_pie(x_scaled, y_scaled + 2, bunker.1.radius * self.particle_size, 180, 360, color).unwrap();
        }
    }

}
