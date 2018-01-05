
use sdl2;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::video::Window;
use sdl2::pixels;

use battlefield;
use battlefield::grid;

const GRID_WIDTH :i32 = 800;
const GRID_HEIGHT:i32 = 500;

pub struct PresenterState {
    canvas: sdl2::render::Canvas<Window>,
}
impl PresenterState {
    pub fn new(sdl_context: &sdl2::Sdl,) -> PresenterState {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Cannonland",
            GRID_WIDTH as u32,
            GRID_HEIGHT as u32)
          .position_centered().resizable()
          .build()
          .unwrap();
        PresenterState {
            canvas: window.into_canvas().build().unwrap(),
        }
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

    pub fn rescale_canvas(&mut self, x: i32, y: i32) {
        self.state.canvas.set_scale
            (x as f32 / GRID_WIDTH  as f32,
             y as f32 / GRID_HEIGHT as f32).unwrap();
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
        for bunker in &self.battlefield.grid.bunkers {
            let cannon_pos: (i16,i16,i16,i16) = bunker.1.get_cannon_pos_x1y1x2y2();
            let x = bunker.1.x_pos;
            let y = bunker.1.y_pos;
            let rgba: (u8,u8,u8,u8) = bunker.0.get_rgba();
            let color = pixels::Color::RGBA(rgba.0, rgba.1, rgba.2, rgba.3);

            self.state.canvas.aa_line(
                cannon_pos.0, cannon_pos.1,
                cannon_pos.2, cannon_pos.3,
                color).unwrap();
            self.state.canvas.filled_pie(x, y, bunker.1.radius, 180, 360, color).unwrap();
        }
    }

}

// draw shots
impl<'st,'g> Presenter<'st,'g> {

    fn draw_shots(&mut self) -> () {
        for shot in self.battlefield.get_shots() {
            self.state.canvas.filled_circle(shot.x_pos as i16, shot.y_pos as i16, 4, pixels::Color::RGBA(96,96,96,255)).unwrap();
        }
    }

}
