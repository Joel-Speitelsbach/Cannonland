use std::time::SystemTime;

use sdl2;
use sdl2::event::{Event,WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::video::Window;
use sdl2::pixels;

use game;
use game::grid::{self,Grid};

const GRID_WIDTH :i32 = 800;
const GRID_HEIGHT:i32 = 500;

pub fn run() {
    let mut game = game::Game::new();
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut window = video_subsystem.window("Cannonland", 
        game.grid.width as u32,
        game.grid.height as u32)
      .position_centered().resizable()
      .build()
      .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    let mut presenter_state = PresenterState::new();
    let mut presenter = Presenter::new(&mut presenter_state, &mut canvas, &mut game);
    
    'mainloop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                Event::Window{win_event: WindowEvent::Resized
                        (width,height),..} =>
                    presenter.rescale_canvas(width,height,),
                _ => {}
            }
        }
        presenter.stride();
    }
}

struct PresenterState {
    fps_manager: sdl2::gfx::framerate::FPSManager,
}
impl PresenterState {
    pub fn new() -> PresenterState {
        let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
        fps_manager.set_framerate(60).unwrap();
        PresenterState {
            fps_manager: fps_manager,
        }
    }
}

struct Presenter<'st,'g> {
    state: &'st mut PresenterState,
    game: &'g mut game::Game,
    canvas: &'g mut sdl2::render::Canvas<Window>,
}
impl<'st,'g> Presenter<'st,'g> {
    pub fn new(
        presenter_state: &'st mut PresenterState,
        canvas: &'g mut sdl2::render::Canvas<Window>,
        game: &'g mut game::Game,
        )-> Presenter<'st,'g> 
    {
        return Presenter{
            state: presenter_state,
            game: game,
            canvas: canvas,
        };
    }

    fn stride(&mut self) -> () {

        let calc_time = SystemTime::now();
        self.grid().stride();
        print!("calc needed {} msecs", calc_time.elapsed().unwrap().subsec_nanos() / (1000*1000));

        let present_time = SystemTime::now();
        self.present_grid();
        print!(", present needed {} msecs", present_time.elapsed().unwrap().subsec_nanos() / (1000*1000));

        println!(", calc and present needed {} msecs", calc_time.elapsed().unwrap().subsec_nanos() / (1000*1000));

        self.state.fps_manager.delay();
    }

    fn rescale_canvas(&mut self, x: i32, y: i32) {
        // let (x,y) = canvas.output_size().unwrap();
        // let x = x as i32;
        // let y = y as i32;
        // let (x,y) = 
            // if x*GRID_HEIGHT < y*GRID_WIDTH
                  // {((y*GRID_WIDTH)/GRID_HEIGHT,y                         )
            // }else {(x                         ,(x*GRID_HEIGHT)/GRID_WIDTH)};
        // canvas.window_mut().set_size(x as u32,y as u32).unwrap();
        self.canvas.set_scale
            (x as f32 / GRID_WIDTH  as f32, 
             y as f32 / GRID_HEIGHT as f32).unwrap();
    }
}

// draw grid
impl<'st,'g> Presenter<'st,'g> {
    fn present_grid(&mut self) -> () {
        self.draw_background();

        self.draw_particles();
        self.draw_bunkers();

        self.canvas.present();
    }
    
    fn grid(&mut self) -> &mut grid::Grid {&mut self.game.grid}

    fn draw_background(&mut self) -> () {
        self.canvas.set_draw_color(pixels::Color::RGBA(96,128,200,255));
        self.canvas.clear();
    }

    fn draw_particles(&mut self) -> () {
        for y in 0..self.grid().height {
            for x in 0..self.grid().width {
                self.draw_particle(x, y);
            }
        }
    }

    fn draw_particle(&mut self, x: usize, y: usize) -> () {
        let rgba: (u8,u8,u8,u8) = self.grid().grid[y][x].get_rgba();

        if rgba.3 != 0 {
            let color = pixels::Color::RGBA(rgba.0, rgba.1, rgba.2, rgba.3);
            let x = x as i16;
            let y = y as i16;
            self.canvas.pixel(x, y, color).unwrap();
        }
    }

    fn draw_bunkers(&mut self) -> () {
        for bunker in &self.game.grid.bunkers {
            let cannon_pos: (i16,i16,i16,i16) = bunker.1.get_cannon_pos_x1y1x2y2();
            let x = bunker.1.x_pos;
            let y = bunker.1.y_pos;
            let rgba: (u8,u8,u8,u8) = bunker.0.get_rgba();
            let color = pixels::Color::RGBA(rgba.0, rgba.1, rgba.2, rgba.3);

            self.canvas.thick_line(
                cannon_pos.0, cannon_pos.1,
                cannon_pos.2, cannon_pos.3,
                2, color).unwrap();
            self.canvas.filled_pie(x, y + 2, bunker.1.radius, 180, 360, color).unwrap();
        }
    }

}
