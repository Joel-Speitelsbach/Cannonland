
use sdl2;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::video::Window;
use sdl2::pixels;
use sdl2::event::{Event,WindowEvent};

use battlefield::{self,grid,Battlefield};

// const GRID_WIDTH :i32 = 800;
// const GRID_HEIGHT:i32 = 500;

pub struct PresenterState {
    canvas: sdl2::render::Canvas<Window>,
}
impl PresenterState {
    pub fn new(sdl_context: &sdl2::Sdl, battlefield: &Battlefield) -> PresenterState {
        let video_subsystem = sdl_context.video().unwrap();
        let window =
            video_subsystem
            .window("Cannonland",
                battlefield.grid.width  as u32,
                battlefield.grid.height as u32)
            .build()
            .unwrap();
        let mut canvas =
            window
            .into_canvas()
            .software()
            .build()
            .unwrap();
        canvas.window_mut().set_size(
            battlefield.grid.width  as u32 * 3,
            battlefield.grid.height as u32 * 3,
        ).unwrap();
        canvas.window_mut().set_position(
            sdl2::video::WindowPos::Centered,
            sdl2::video::WindowPos::Centered);
        PresenterState { canvas }
    }
}

pub struct Presenter<'st,'g> {
    state: &'st mut PresenterState,
    battlefield: &'g mut battlefield::Battlefield,
}
impl<'st,'g> Presenter<'st,'g> {
    pub fn new(
        presenter_state: &'st mut PresenterState,
        battlefield: &'g mut battlefield::Battlefield,
        )-> Presenter<'st,'g>
    {
        Presenter{
            state: presenter_state,
            battlefield: battlefield,
        }
    }

    pub fn present(&mut self) -> () {
        self.draw_grid();
        self.draw_bunkers();
        self.draw_shots();
        self.state.canvas.present();
    }

    pub fn respond_to(&mut self, event: &Event) {
        match *event {
            Event::Window{win_event: WindowEvent::Resized
                    (width,height),..} =>
                self.rescale_canvas(width,height,),
            _ => (),
        }
    }

    fn rescale_canvas(&mut self, x: i32, y: i32) {
        self.state.canvas.set_scale
            (x as f32 / self.battlefield.grid.width  as f32,
             y as f32 / self.battlefield.grid.height as f32).unwrap();
    }
}

// draw grid
impl<'st,'g> Presenter<'st,'g> {
    fn draw_grid(&mut self) -> () {
        self.draw_background();
        self.draw_particles();
    }

    fn grid(&mut self) -> &mut grid::Grid {&mut self.battlefield.grid}

    fn draw_background(&mut self) -> () {
        self.state.canvas.set_draw_color(pixels::Color::RGBA(96,128,200,255));
        self.state.canvas.clear();
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
            self.state.canvas.pixel(x, y, color).unwrap();
        }
    }

}

// draw bunkers
impl<'st,'g> Presenter<'st,'g> {

    fn draw_bunkers(&mut self) -> () {
        for bunker in &self.battlefield.bunkers {
            let rgba: (u8,u8,u8,u8) = bunker.get_rgba();
            let color = pixels::Color::RGBA(rgba.0, rgba.1, rgba.2, rgba.3);

            Presenter::draw_cannon(&mut self.state.canvas, &bunker, color);
            Presenter::draw_building(&mut self.state.canvas, &bunker, color);
            Presenter::draw_charge(&mut self.state.canvas, &bunker);
            Presenter::draw_health(&mut self.state.canvas, &bunker);
        }
    }

    fn draw_cannon(canvas: &mut sdl2::render::Canvas<Window>, bunker: &battlefield::bunker::Bunker, color: pixels::Color) {
        let cannon_pos: (i16,i16,i16,i16) = bunker.get_cannon_pos_x1y1x2y2();
        canvas.aa_line(
            cannon_pos.0, cannon_pos.1,
            cannon_pos.2, cannon_pos.3,
            color).unwrap();
    }

    fn draw_building(canvas: &mut sdl2::render::Canvas<Window>, bunker: &battlefield::bunker::Bunker, color: pixels::Color) {
        canvas.filled_pie(bunker.x_pos, bunker.y_pos, bunker.get_radius() as i16, 180, 360, color).unwrap();
    }

    fn draw_charge(canvas: &mut sdl2::render::Canvas<Window>, bunker: &battlefield::bunker::Bunker) {
        let divisor = 4;

        let y1 = bunker.y_pos + 1;
        let y2 = y1 + 5;
        let x_zero = bunker.x_pos - (bunker.get_max_charge() as i16/2/divisor);
        let x_current = x_zero + bunker.get_charge() as i16/divisor;
        let x_max = x_zero + bunker.get_max_charge() as i16/divisor;

        canvas.box_(x_zero, y1, x_max, y2, pixels::Color::RGBA(128,128,128,128)).unwrap();
        canvas.box_(x_zero, y1, x_current, y2, pixels::Color::RGBA(0,0,255,255)).unwrap();
    }

    fn draw_health(canvas: &mut sdl2::render::Canvas<Window>, bunker: &battlefield::bunker::Bunker) {
        let divisor = 4;

        let y1 = bunker.y_pos + 7;
        let y2 = y1 + 5;
        let x_zero = bunker.x_pos - (bunker.get_max_health() as i16/2/divisor);
        let x_current = x_zero + bunker.get_health() as i16/divisor;
        let x_max = x_zero + bunker.get_max_health() as i16/divisor;

        canvas.box_(x_zero, y1, x_max, y2, pixels::Color::RGBA(255,0,0,128)).unwrap();
        canvas.box_(x_zero, y1, x_current, y2, pixels::Color::RGBA(0,255,0,255)).unwrap();
    }

}

// draw shots
impl<'st,'g> Presenter<'st,'g> {

    fn draw_shots(&mut self) -> () {
        for shot in &self.battlefield.shots {
            self.state.canvas.filled_circle(shot.x_pos as i16, shot.y_pos as i16, 4, pixels::Color::RGBA(96,96,96,255)).unwrap();
        }
    }

}
